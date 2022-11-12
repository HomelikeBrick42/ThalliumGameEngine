use std::{
    collections::HashMap,
    ffi::{c_void, CString},
    marker::PhantomData,
    mem::size_of,
    pin::Pin,
    sync::atomic::AtomicBool,
};

use lazy_static::lazy_static;
use widestring::U16CString;
use windows::{
    core::{PCSTR, PCWSTR},
    Win32::{
        Foundation::HINSTANCE,
        Graphics::{
            Gdi::{GetDC, HDC},
            OpenGL::{
                wglCreateContext, wglDeleteContext, wglGetProcAddress, wglMakeCurrent,
                ChoosePixelFormat, SetPixelFormat, SwapBuffers, HGLRC, PFD_DOUBLEBUFFER,
                PFD_DRAW_TO_WINDOW, PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA,
                PIXELFORMATDESCRIPTOR,
            },
        },
        System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryW},
    },
};

use crate::{
    math::{Matrix4x4, Vector2, Vector3},
    renderer::{
        opengl::{OpenGLShader, OpenGLTexture, OpenGLVertexBuffer},
        IndexBuffer, IndexBufferID, Pixels, Renderer, RendererDrawContext, Shader, ShaderID,
        Texture, TextureID, VertexBuffer, VertexBufferElement, VertexBufferID,
    },
    scene::Camera,
    PhantomUnsend, PhantomUnsync, Window,
};

use super::OpenGLIndexBuffer;

pub(crate) struct OpenGLRenderer {
    window: Option<Pin<Box<Window>>>,
    opengl_library: HINSTANCE,
    device_context: HDC,
    opengl_context: HGLRC,
    shaders: HashMap<ShaderID, OpenGLShader>,
    vertex_buffers: HashMap<VertexBufferID, OpenGLVertexBuffer>,
    index_buffers: HashMap<IndexBufferID, OpenGLIndexBuffer>,
    textures: HashMap<TextureID, OpenGLTexture>,
    default_white_pixel: OpenGLTexture,
    _send: PhantomUnsend,
    _sync: PhantomUnsync,
}

lazy_static! {
    static ref CONTEXT_CREATED: AtomicBool = AtomicBool::new(false);
}

impl OpenGLRenderer {
    pub(crate) fn new(window: Pin<Box<Window>>) -> OpenGLRenderer {
        if CONTEXT_CREATED.swap(true, std::sync::atomic::Ordering::AcqRel) {
            panic!(
                "Can only create 1 opengl context at a time, current limitation of the gl crate"
            );
        }

        let dll_name = U16CString::from_str("opengl32.dll").unwrap();
        let opengl_library = unsafe { LoadLibraryW(PCWSTR(dll_name.as_ptr())) }.unwrap();

        let device_context = unsafe { GetDC(window.window_handle) };
        if device_context == HDC::default() {
            panic!("Failed to get device context");
        }

        let pixel_format_descriptor = PIXELFORMATDESCRIPTOR {
            nSize: size_of::<PIXELFORMATDESCRIPTOR>() as _,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cDepthBits: 24,
            cStencilBits: 8,
            iLayerType: PFD_MAIN_PLANE,
            ..Default::default()
        };

        let format = unsafe { ChoosePixelFormat(device_context, &pixel_format_descriptor) };
        if format == 0 {
            panic!("Could not find format");
        }

        if unsafe { SetPixelFormat(device_context, format, &pixel_format_descriptor) } == false {
            panic!("Failed to set pixel format");
        }

        let temp_opengl_context = unsafe { wglCreateContext(device_context) }.unwrap();
        if temp_opengl_context == HGLRC::default() {
            panic!("Failed to create temp opengl context current");
        }

        if unsafe { wglMakeCurrent(device_context, temp_opengl_context) } == false {
            panic!("Failed to bind temp opengl context");
        }

        #[allow(non_snake_case)]
        let wglCreateContextAttribsARB: extern "C" fn(
            hDC: HDC,
            hshareContext: HGLRC,
            attribList: *const i32,
        ) -> HGLRC = unsafe {
            std::mem::transmute(wglGetProcAddress(PCSTR(
                b"wglCreateContextAttribsARB\0".as_ptr(),
            )))
        };

        let attribs = [
            0x2091, 3, // WGL_CONTEXT_MAJOR_VERSION_ARB
            0x2092, 3, // WGL_CONTEXT_MINOR_VERSION_ARB
            0x9126, 1, // WGL_CONTEXT_PROFILE_MASK_ARB WGL_CONTEXT_CORE_PROFILE_BIT_ARB
            0,
        ];

        let opengl_context =
            wglCreateContextAttribsARB(device_context, HGLRC::default(), attribs.as_ptr());
        if opengl_context == HGLRC::default() {
            panic!("Failed to create opengl context current");
        }

        if unsafe { wglMakeCurrent(device_context, opengl_context) } == false {
            panic!("Failed to bind opengl context");
        }

        if unsafe { wglDeleteContext(temp_opengl_context) } == false {
            panic!("Failed to destroy temp opengl context");
        }

        gl::load_with(|s| unsafe {
            let cstr = CString::new(s).unwrap();
            let mut ptr: *const c_void =
                std::mem::transmute(wglGetProcAddress(PCSTR(cstr.as_ptr() as _)));
            if ptr.is_null() {
                ptr =
                    std::mem::transmute(GetProcAddress(opengl_library, PCSTR(cstr.as_ptr() as _)));
            }
            ptr
        });

        OpenGLRenderer {
            window: Some(window),
            opengl_library,
            device_context,
            opengl_context,
            shaders: HashMap::new(),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            textures: HashMap::new(),
            default_white_pixel: OpenGLTexture::new(
                (1, 1).into(),
                Pixels::RGBA(&[(255, 255, 255, 255).into()]),
            ),
            _send: PhantomData,
            _sync: PhantomData,
        }
    }

    fn destroy(&mut self) {
        if unsafe { wglMakeCurrent(self.device_context, HGLRC::default()) } == false {
            panic!("Failed to unbind opengl context");
        }
        if unsafe { wglDeleteContext(self.opengl_context) } == false {
            panic!("Failed to destroy opengl context");
        }

        if unsafe { FreeLibrary(self.opengl_library) } == false {
            panic!("Unable to unload opengl32.dll");
        }

        CONTEXT_CREATED.store(false, std::sync::atomic::Ordering::Release);
    }
}

impl Drop for OpenGLRenderer {
    fn drop(&mut self) {
        if self.window.is_some() {
            self.destroy();
        }
    }
}

impl Renderer for OpenGLRenderer {
    fn get_window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }

    fn get_window_mut(&mut self) -> &mut Window {
        self.window.as_mut().unwrap()
    }

    fn take_window(mut self) -> Pin<Box<Window>> {
        self.destroy();
        self.window.take().unwrap()
    }

    fn create_shader(
        &mut self,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<ShaderID, String> {
        let shader = OpenGLShader::new(vertex_shader_source, fragment_shader_source)?;
        let id = shader.get_id();
        assert!(self.shaders.insert(id, shader).is_none());
        Ok(id)
    }

    fn destroy_shader(&mut self, id: ShaderID) {
        self.shaders.remove(&id);
    }

    fn get_shader(&self, id: ShaderID) -> Option<&dyn Shader> {
        self.shaders.get(&id).map(|shader| shader as &dyn Shader)
    }

    fn get_shader_mut(&mut self, id: ShaderID) -> Option<&mut dyn Shader> {
        self.shaders
            .get_mut(&id)
            .map(|shader| shader as &mut dyn Shader)
    }

    fn create_vertex_buffer(
        &mut self,
        layout: &[VertexBufferElement],
        data: &[u8],
    ) -> VertexBufferID {
        let vertex_buffer = OpenGLVertexBuffer::new(layout, data);
        let id = vertex_buffer.get_id();
        assert!(self.vertex_buffers.insert(id, vertex_buffer).is_none());
        id
    }

    fn destroy_vertex_buffer(&mut self, id: VertexBufferID) {
        self.vertex_buffers.remove(&id);
    }

    fn get_vertex_buffer(&self, id: VertexBufferID) -> Option<&dyn VertexBuffer> {
        self.vertex_buffers
            .get(&id)
            .map(|vertex_buffer| vertex_buffer as &dyn VertexBuffer)
    }

    fn get_vertex_buffer_mut(&mut self, id: VertexBufferID) -> Option<&mut dyn VertexBuffer> {
        self.vertex_buffers
            .get_mut(&id)
            .map(|vertex_buffer| vertex_buffer as &mut dyn VertexBuffer)
    }

    fn create_index_buffer(&mut self, indices: &[u32]) -> IndexBufferID {
        let index_buffer = OpenGLIndexBuffer::new(indices);
        let id = index_buffer.get_id();
        assert!(self.index_buffers.insert(id, index_buffer).is_none());
        id
    }

    fn destroy_index_buffer(&mut self, id: IndexBufferID) {
        self.index_buffers.remove(&id);
    }

    fn get_index_buffer(&self, id: IndexBufferID) -> Option<&dyn IndexBuffer> {
        self.index_buffers
            .get(&id)
            .map(|index_buffer| index_buffer as &dyn IndexBuffer)
    }

    fn get_index_buffer_mut(&mut self, id: IndexBufferID) -> Option<&mut dyn IndexBuffer> {
        self.index_buffers
            .get_mut(&id)
            .map(|index_buffer| index_buffer as &mut dyn IndexBuffer)
    }

    fn create_texture(&mut self, size: Vector2<usize>, data: Pixels) -> TextureID {
        let texture = OpenGLTexture::new(size, data);
        let id = texture.get_id();
        assert!(self.textures.insert(id, texture).is_none());
        id
    }

    fn destroy_texture(&mut self, id: TextureID) {
        self.textures.remove(&id);
    }

    fn get_texture(&self, id: TextureID) -> Option<&dyn Texture> {
        self.textures
            .get(&id)
            .map(|texture| texture as &dyn Texture)
    }

    fn get_texture_mut(&mut self, id: TextureID) -> Option<&mut dyn Texture> {
        self.textures
            .get_mut(&id)
            .map(|texture| texture as &mut dyn Texture)
    }

    fn resize(&mut self, size: Vector2<usize>) {
        unsafe { gl::Viewport(0, 0, size.x as _, size.y as _) }
    }

    fn present(&mut self) {
        unsafe { SwapBuffers(self.device_context) };
    }

    fn clear(&mut self, color: Vector3<f32>) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }

    fn drawing_context<'a>(
        &'a mut self,
        camera: Camera<f32>,
        depth_testing: bool,
    ) -> Box<dyn RendererDrawContext + 'a> {
        unsafe {
            if depth_testing {
                gl::Enable(gl::DEPTH_TEST);
                gl::DepthFunc(gl::GEQUAL);
                gl::ClearDepth(0.0);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }
        }
        Box::new(OpenGLRendererDrawContext {
            renderer: self,
            view_matrix: camera.transform.into(),
            projection_matrix: camera.projection_type.into(),
            _send: PhantomData,
            _sync: PhantomData,
        })
    }
}

pub struct OpenGLRendererDrawContext<'a> {
    renderer: &'a mut OpenGLRenderer,
    view_matrix: Matrix4x4<f32>,
    projection_matrix: Matrix4x4<f32>,
    _send: PhantomUnsend,
    _sync: PhantomUnsync,
}

impl<'a> RendererDrawContext for OpenGLRendererDrawContext<'a> {
    fn draw(
        &mut self,
        shader: ShaderID,
        vertex_buffer: VertexBufferID,
        texture: Option<TextureID>,
        model_matrix: Matrix4x4<f32>,
        color: Vector3<f32>,
    ) {
        // TODO: maybe some proper error handling
        let Some(shader) = self.renderer.shaders.get_mut(&shader) else { return; };
        let Some(vertex_buffer) = self.renderer.vertex_buffers.get_mut(&vertex_buffer) else { return; };

        shader.bind();
        vertex_buffer.bind();
        let texture_index = 0;
        if let Some(texture) = texture
            .map(|id| self.renderer.textures.get_mut(&id))
            .flatten()
        {
            texture.bind(texture_index);
        } else {
            self.renderer.default_white_pixel.bind(texture_index);
        }
        unsafe {
            shader.set_uniform_matrix("u_ProjectionMatrix", &self.projection_matrix);
            shader.set_uniform_matrix("u_ViewMatrix", &self.view_matrix);
            shader.set_uniform_matrix("u_ModelMatrix", &model_matrix);
            shader.set_uniform_vector3("u_Color", color);
            shader.set_uniform_uint("u_Texture", texture_index);
            assert_eq!(vertex_buffer.get_count() % 3, 0);
            gl::DrawArrays(gl::TRIANGLES, 0, vertex_buffer.get_count() as _);
        }
        if let Some(texture) = texture
            .map(|id| self.renderer.textures.get_mut(&id))
            .flatten()
        {
            texture.unbind();
        }
        vertex_buffer.unbind();
        shader.unbind();
    }

    fn draw_indexed(
        &mut self,
        shader: ShaderID,
        vertex_buffer: VertexBufferID,
        index_buffer: IndexBufferID,
        texture: Option<TextureID>,
        model_matrix: Matrix4x4<f32>,
        color: Vector3<f32>,
    ) {
        // TODO: maybe some proper error handling
        let Some(shader) = self.renderer.shaders.get_mut(&shader) else { return; };
        let Some(vertex_buffer) = self.renderer.vertex_buffers.get_mut(&vertex_buffer) else { return; };
        let Some(index_buffer) = self.renderer.index_buffers.get_mut(&index_buffer) else { return; };

        shader.bind();
        vertex_buffer.bind();
        index_buffer.bind();
        let texture_index = 0;
        if let Some(texture) = texture
            .map(|id| self.renderer.textures.get_mut(&id))
            .flatten()
        {
            texture.bind(texture_index);
        } else {
            self.renderer.default_white_pixel.bind(texture_index);
        }
        unsafe {
            shader.set_uniform_matrix("u_ProjectionMatrix", &self.projection_matrix);
            shader.set_uniform_matrix("u_ViewMatrix", &self.view_matrix);
            shader.set_uniform_matrix("u_ModelMatrix", &model_matrix);
            shader.set_uniform_vector3("u_Color", color);
            shader.set_uniform_uint("u_Texture", texture_index);
            assert_eq!(index_buffer.get_count() % 3, 0);
            gl::DrawElements(
                gl::TRIANGLES,
                index_buffer.get_count() as _,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
        if let Some(texture) = texture
            .map(|id| self.renderer.textures.get_mut(&id))
            .flatten()
        {
            texture.unbind();
        }
        index_buffer.unbind();
        vertex_buffer.unbind();
        shader.unbind();
    }
}
