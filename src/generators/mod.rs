use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, ListBoxRow, GestureClick};

// Expose generators
mod generator_url;
pub use generator_url::url_generator;
mod generator_email;
pub use generator_email::email_generator;

pub struct QrGenerator {
    pub disp: Box,
    pub row: ListBoxRow,
}

impl QrGenerator {
    pub fn new(name: &'static str) -> Self {
        // disp
        let my_box = Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .margin_bottom(10)
            .margin_top(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        // row
        let my_row = ListBoxRow::new();
        let my_label = Label::builder()
            .label(name)
            .margin_bottom(10)
            .margin_top(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        let gesture = GestureClick::new();
        gesture.set_button(gtk4::gdk::ffi::GDK_BUTTON_PRIMARY as u32);
        gesture.connect_pressed(move |_,_,_,_| {
            println!("Selected item: {}", name);
        });

        my_row.add_controller(gesture);
        my_row.set_child(Some(&my_label));

        // Return self
        Self {
            disp: my_box,
            row: my_row
        }
    }
}