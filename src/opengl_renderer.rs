use std::{
    ffi::{c_void, CString},
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

use crate::{Renderer, Window};

pub(crate) struct OpenGLRenderer {
    window: Option<Pin<Box<Window>>>,
    opengl_library: HINSTANCE,
    device_context: HDC,
    opengl_context: HGLRC,
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
            0x2091, 4, // WGL_CONTEXT_MAJOR_VERSION_ARB
            0x2092, 4, // WGL_CONTEXT_MAJOR_VERSION_ARB
            0x9126, 1, // WGL_CONTEXT_PROFILE_MASK_ARB WGL_CONTEXT_CORE_PROFILE_BIT_ARB
            0x2094, 1, // WGL_CONTEXT_FLAGS_ARB WGL_CONTEXT_DEBUG_BIT_ARB
            0,
        ];

        let opengl_context =
            wglCreateContextAttribsARB(device_context, HGLRC::default(), attribs.as_ptr());
        if temp_opengl_context == HGLRC::default() {
            panic!("Failed to create opengl context current");
        }

        if unsafe { wglMakeCurrent(device_context, temp_opengl_context) } == false {
            panic!("Failed to bind opengl context");
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

    fn clear(&mut self, color: crate::Vector3<f32>) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }

    fn present(&mut self) {
        unsafe { SwapBuffers(self.device_context) };
    }
}

impl Drop for OpenGLRenderer {
    fn drop(&mut self) {
        if self.window.is_some() {
            self.destroy();
        }
    }
}