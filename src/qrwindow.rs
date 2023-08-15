use gtk::prelude::*;
use gtk::{Window, WindowType, Box, ListBox, Label, ListBoxRow, Orientation};

pub struct QrgenWindow {
    my_window: Window,
    my_box: Box,
    my_list: ListBox,
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
            my_window: new_window,
            my_box: Box::new(Orientation::Horizontal, 0),
            my_list: ListBox::new()
        }
    }

    pub fn add_item(self, text: &str) -> Self {
        let my_row = ListBoxRow::new();
        let my_label = Label::builder()
            .label(text)
            .margin(10)
            .build();

        my_row.add(&my_label);
        self.my_list.add(&my_row);
        // Return self
        self
    }

    pub fn build(self) -> Window {
        self.my_box.add(&self.my_list);
        self.my_window.add(&self.my_box);

        // Return QrgenWindow
        self.my_window
    }
}