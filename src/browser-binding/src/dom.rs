//! Интерфейс взаимодействия с DOM

use web_sys::{Document, Window};
pub(crate) struct DOM {
    window: Window,
    document: Document
}

impl DOM {
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("no `document` on window");
        Self {
            window,
            document
        }
    }
    pub fn get_window(&self) -> &Window {
        &self.window
    }
    pub fn get_document(&self) -> &Document {
        &self.document
    }
}

