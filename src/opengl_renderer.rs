use std::pin::Pin;

use crate::{Renderer, Window};

pub(crate) struct OpenGLRenderer {
    window: Option<Pin<Box<Window>>>,
}

impl OpenGLRenderer {
    pub(crate) fn new(window: Pin<Box<Window>>) -> OpenGLRenderer {
        OpenGLRenderer {
            window: Some(window),
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
        self.window.take().unwrap()
    }
}

impl Drop for OpenGLRenderer {
    fn drop(&mut self) {}
}
