use std::{
    mem::size_of,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
};

use enum_map::EnumMap;
use widestring::U16CString;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Input::KeyboardAndMouse::{
                VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A,
                VK_B, VK_C, VK_CONTROL, VK_D, VK_DOWN, VK_E, VK_F, VK_G, VK_H, VK_I, VK_J, VK_K,
                VK_L, VK_LEFT, VK_LMENU, VK_M, VK_N, VK_O, VK_P, VK_Q, VK_R, VK_RIGHT, VK_S,
                VK_SHIFT, VK_T, VK_U, VK_UP, VK_V, VK_W, VK_X, VK_Y, VK_Z,
            },
            WindowsAndMessaging::{
                AdjustWindowRectEx, CreateWindowExW, DefWindowProcW, DestroyWindow,
                DispatchMessageW, GetClientRect, GetWindowLongPtrW, LoadCursorW, PeekMessageW,
                RegisterClassExW, SetWindowLongPtrW, ShowWindow, TranslateMessage,
                UnregisterClassW, CREATESTRUCTW, CS_OWNDC, CW_USEDEFAULT, GWLP_USERDATA, HMENU,
                IDC_ARROW, MSG, PM_REMOVE, SW_HIDE, SW_SHOW, WINDOW_EX_STYLE, WM_CLOSE, WM_KEYDOWN,
                WM_KEYUP, WM_NCCREATE, WM_QUIT, WM_SIZE, WM_SYSKEYDOWN, WM_SYSKEYUP, WNDCLASSEXW,
                WS_OVERLAPPEDWINDOW,
            },
        },
    },
};

use crate::{
    math::Vector2,
    platform::{Keycode, SurfaceEvent},
    renderer::{new_renderer, Renderer, RendererAPI},
};

pub struct Surface {
    instance: HINSTANCE,
    window_class_name: U16CString,
    size: Vector2<usize>,
    pub(crate) window_handle: HWND,
    events: Vec<SurfaceEvent>,
    key_states: EnumMap<Keycode, bool>,
}

impl Surface {
    pub fn new(size: Vector2<usize>, title: &str) -> Pin<Box<Surface>> {
        let instance = unsafe { GetModuleHandleW(PCWSTR::null()) }.unwrap();

        let window_class_name = {
            pub static WINDOW_CLASS_ID: AtomicUsize = AtomicUsize::new(1);
            U16CString::from_str(format!(
                "TestGameEngineWindow{}",
                WINDOW_CLASS_ID.fetch_add(1, Ordering::AcqRel)
            ))
            .unwrap()
        };

        if unsafe {
            RegisterClassExW(&WNDCLASSEXW {
                cbSize: size_of::<WNDCLASSEXW>() as _,
                style: CS_OWNDC,
                lpfnWndProc: Some(window_message_callback),
                hInstance: instance,
                hCursor: LoadCursorW(HINSTANCE::default(), IDC_ARROW).unwrap(),
                lpszClassName: PCWSTR(window_class_name.as_ptr()),
                ..Default::default()
            })
        } == 0
        {
            panic!("Failed to register window class");
        }

        let window_style = WS_OVERLAPPEDWINDOW;
        let window_style_ex = WINDOW_EX_STYLE::default();

        let mut rect = RECT::default();
        rect.left = 100;
        rect.right = rect.left + size.x as i32;
        rect.top = 100;
        rect.bottom = rect.top + size.y as i32;
        if unsafe { AdjustWindowRectEx(&mut rect, window_style, false, window_style_ex) } == false {
            panic!("Failed to calculate window bounds");
        }

        let width = (rect.right - rect.left) as usize;
        let height = (rect.bottom - rect.top) as usize;

        let mut surface = Pin::new(Box::new(Surface {
            instance,
            window_class_name,
            size,
            window_handle: HWND::default(),
            events: vec![],
            key_states: EnumMap::default(),
        }));

        let window_title = U16CString::from_str(title).unwrap();
        surface.window_handle = unsafe {
            CreateWindowExW(
                window_style_ex,
                PCWSTR(surface.window_class_name.as_ptr()),
                PCWSTR(window_title.as_ptr()),
                window_style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as _,
                height as _,
                HWND::default(),
                HMENU::default(),
                surface.instance,
                Some(&surface as &Surface as *const _ as _),
            )
        };
        if surface.window_handle == HWND::default() {
            panic!("Failed to create window");
        }

        surface
    }

    pub fn show(&mut self) {
        unsafe { ShowWindow(self.window_handle, SW_SHOW) };
    }
    pub fn hide(&mut self) {
        unsafe { ShowWindow(self.window_handle, SW_HIDE) };
    }

    pub fn get_size(&self) -> Vector2<usize> {
        self.size
    }

    pub fn get_key_state(&self, key: Keycode) -> bool {
        self.key_states[key]
    }

    pub fn events(&mut self) -> impl Iterator<Item = SurfaceEvent> {
        unsafe {
            let mut message = MSG::default();
            while PeekMessageW(&mut message, self.window_handle, 0, 0, PM_REMOVE) != false {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
            std::mem::take(&mut self.events).into_iter()
        }
    }

    pub fn into_renderer(self: Pin<Box<Surface>>, api: RendererAPI) -> Box<dyn Renderer> {
        new_renderer(self, api)
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            if DestroyWindow(self.window_handle) == false {
                panic!("Failed to destroy the window");
            }
            if UnregisterClassW(PCWSTR(self.window_class_name.as_ptr()), self.instance) == false {
                panic!("Failed to unregister window class");
            }
        }
    }
}

unsafe extern "system" fn window_message_callback(
    hwnd: HWND,
    message: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if message == WM_NCCREATE {
        let createstruct: *const CREATESTRUCTW = std::mem::transmute(l_param);
        SetWindowLongPtrW(
            hwnd,
            GWLP_USERDATA,
            std::mem::transmute((*createstruct).lpCreateParams),
        );
        return DefWindowProcW(hwnd, message, w_param, l_param);
    }

    let surface: *mut Surface = std::mem::transmute(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
    if surface.is_null() {
        return DefWindowProcW(hwnd, message, w_param, l_param);
    }
    let surface = &mut *surface;

    let mut result = LRESULT::default();
    match message {
        WM_QUIT | WM_CLOSE => surface.events.push(SurfaceEvent::Close),
        WM_SIZE => {
            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            if width > 0 && height > 0 {
                surface.size = (width as _, height as _).into();
                surface.events.push(SurfaceEvent::Resize(surface.size));
            }
        }
        WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => 'key_handling: {
            let pressed = matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN);
            let keycode = match VIRTUAL_KEY(w_param.0 as _) {
                VK_0 => Keycode::Num0,
                VK_1 => Keycode::Num1,
                VK_2 => Keycode::Num2,
                VK_3 => Keycode::Num3,
                VK_4 => Keycode::Num4,
                VK_5 => Keycode::Num5,
                VK_6 => Keycode::Num6,
                VK_7 => Keycode::Num7,
                VK_8 => Keycode::Num8,
                VK_9 => Keycode::Num9,
                VK_A => Keycode::A,
                VK_B => Keycode::B,
                VK_C => Keycode::C,
                VK_D => Keycode::D,
                VK_E => Keycode::E,
                VK_F => Keycode::F,
                VK_G => Keycode::G,
                VK_H => Keycode::H,
                VK_I => Keycode::I,
                VK_J => Keycode::J,
                VK_K => Keycode::K,
                VK_L => Keycode::L,
                VK_M => Keycode::M,
                VK_N => Keycode::N,
                VK_O => Keycode::O,
                VK_P => Keycode::P,
                VK_Q => Keycode::Q,
                VK_R => Keycode::R,
                VK_S => Keycode::S,
                VK_T => Keycode::T,
                VK_U => Keycode::U,
                VK_V => Keycode::V,
                VK_W => Keycode::W,
                VK_X => Keycode::X,
                VK_Y => Keycode::Y,
                VK_Z => Keycode::Z,
                VK_LEFT => Keycode::Left,
                VK_UP => Keycode::Up,
                VK_RIGHT => Keycode::Right,
                VK_DOWN => Keycode::Down,
                VK_CONTROL => Keycode::Control,
                VK_SHIFT => Keycode::Shift,
                VK_LMENU => Keycode::Alt,
                _ => break 'key_handling,
            };
            surface.key_states[keycode] = pressed;
            surface.events.extend(
                std::iter::repeat_with(|| {
                    if pressed {
                        SurfaceEvent::KeyPressed(keycode)
                    } else {
                        SurfaceEvent::KeyReleased(keycode)
                    }
                })
                .take((l_param.0 & 0xFF) as _),
            );
        }
        _ => result = DefWindowProcW(hwnd, message, w_param, l_param),
    }
    result
}
