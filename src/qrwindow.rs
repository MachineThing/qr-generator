use gtk::prelude::*;
use gtk::{Window, WindowType};

pub struct QrgenWindow {
    myWindow: Window,
}

impl QrgenWindow {
    pub fn new() -> Self{
        let new_window = Window::new(WindowType::Toplevel);
        new_window.set_title("QR Code Generator");
        new_window.set_default_size(750, 500);

        new_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            false.into()
        });

        // Return self
        Self {
            myWindow: new_window
        }
    }

    pub fn build(self) -> Window {
        // Return QrgenWindow
        self.myWindow
    }
}