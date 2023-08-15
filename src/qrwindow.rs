use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Application, Box, ListBox, Orientation};

use crate::generators;
use generators::QrGenerator;

pub struct QrgenWindow {
    my_window: ApplicationWindow,
    my_box: Box,
    my_list: ListBox
}

impl QrgenWindow {
    pub fn new(app: &Application) -> Self {
        let new_window = ApplicationWindow::builder()
            .application(app)
            .title("QR Code Generator")
            .default_width(750)
            .default_height(500)
            .build();

        // Return self
        Self {
            my_window: new_window,
            my_box: Box::new(Orientation::Horizontal, 0),
            my_list: ListBox::new()
        }
    }

    pub fn add_item(self, generator: QrGenerator) -> Self {
        self.my_list.append(&generator.row);

        // Return self
        self
    }

    pub fn build(self) -> ApplicationWindow {
        self.my_box.append(&self.my_list);
        self.my_window.set_child(Some(&self.my_box));

        // Return QrgenWindow
        self.my_window
    }
}