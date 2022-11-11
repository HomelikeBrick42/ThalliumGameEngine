use std::{
    mem::size_of,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
};

use enum_map::{Enum, EnumMap};
use widestring::U16CString;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            AdjustWindowRectEx, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
            GetClientRect, GetWindowLongPtrW, LoadCursorW, PeekMessageW, RegisterClassExW,
            SetWindowLongPtrW, ShowWindow, TranslateMessage, UnregisterClassW, CREATESTRUCTW,
            CS_OWNDC, CW_USEDEFAULT, GWLP_USERDATA, HMENU, IDC_ARROW, MSG, PM_REMOVE, SW_HIDE,
            SW_SHOW, WINDOW_EX_STYLE, WM_CLOSE, WM_KEYDOWN, WM_KEYUP, WM_NCCREATE, WM_QUIT,
            WM_SIZE, WM_SYSKEYDOWN, WM_SYSKEYUP, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
        },
    },
};

use crate::{
    math::Vector2,
    renderer::{new_renderer, Renderer, RendererAPI},
};

pub enum WindowEvent {
    Close,
    Resize(Vector2<usize>),
    KeyPressed(Keycode),
    KeyReleased(Keycode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum)]
pub enum Keycode {
    W,
    A,
    S,
    D,
    Q,
    E,
}

pub struct Window {
    instance: HINSTANCE,
    window_class_name: U16CString,
    size: Vector2<usize>,
    pub(crate) window_handle: HWND,
    events: Vec<WindowEvent>,
    key_states: EnumMap<Keycode, bool>,
}

impl Window {
    pub fn new(size: Vector2<usize>, title: &str) -> Pin<Box<Window>> {
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

        let mut window = Pin::new(Box::new(Window {
            instance,
            window_class_name,
            size,
            window_handle: HWND::default(),
            events: vec![],
            key_states: EnumMap::default(),
        }));

        let window_title = U16CString::from_str(title).unwrap();
        window.window_handle = unsafe {
            CreateWindowExW(
                window_style_ex,
                PCWSTR(window.window_class_name.as_ptr()),
                PCWSTR(window_title.as_ptr()),
                window_style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as _,
                height as _,
                HWND::default(),
                HMENU::default(),
                window.instance,
                Some(&window as &Window as *const _ as _),
            )
        };
        if window.window_handle == HWND::default() {
            panic!("Failed to create window");
        }

        window
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

    pub fn events(&mut self) -> impl Iterator<Item = WindowEvent> {
        unsafe {
            let mut message = MSG::default();
            while PeekMessageW(&mut message, self.window_handle, 0, 0, PM_REMOVE) != false {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
            std::mem::take(&mut self.events).into_iter()
        }
    }

    pub fn into_renderer(self: Pin<Box<Window>>, api: RendererAPI) -> Box<dyn Renderer> {
        new_renderer(self, api)
    }
}

impl Drop for Window {
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

    let window: *mut Window = std::mem::transmute(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
    if window.is_null() {
        return DefWindowProcW(hwnd, message, w_param, l_param);
    }
    let window = &mut *window;

    let mut result = LRESULT::default();
    match message {
        WM_QUIT | WM_CLOSE => window.events.push(WindowEvent::Close),
        WM_SIZE => {
            let mut rect = RECT::default();
            GetClientRect(hwnd, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            if width > 0 && height > 0 {
                window.size = (width as _, height as _).into();
                window.events.push(WindowEvent::Resize(window.size));
            }
        }
        WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => 'key_handling: {
            let pressed = matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN);
            let keycode = match w_param.0 {
                0x41 => Keycode::A,
                0x44 => Keycode::D,
                0x45 => Keycode::E,
                0x51 => Keycode::Q,
                0x53 => Keycode::S,
                0x57 => Keycode::W,
                _ => break 'key_handling,
            };
            window.key_states[keycode] = pressed;
            window.events.extend(
                std::iter::repeat_with(|| {
                    if pressed {
                        WindowEvent::KeyPressed(keycode)
                    } else {
                        WindowEvent::KeyReleased(keycode)
                    }
                })
                .take((l_param.0 & 0xFF) as _),
            );
        }
        _ => result = DefWindowProcW(hwnd, message, w_param, l_param),
    }
    result
}
