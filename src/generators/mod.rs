use gtk4::prelude::*;
use gtk4::{Orientation, Label, ListBoxRow};

// Expose generators
mod generator_url;
pub use generator_url::url_generator;
mod generator_email;
pub use generator_email::email_generator;

pub struct QrGenerator {
    pub disp: gtk4::Box,
    pub row: ListBoxRow,

    pub clear: Box<dyn Fn()>,
    pub generate: Box<dyn Fn() -> String>
}

impl QrGenerator {
    pub fn new(name: &str) -> Self {
        // disp
        let my_box = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
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

        my_row.set_child(Some(&my_label));

        // Return self
        Self {
            disp: my_box,
            row: my_row,
            clear: Box::new(|| {
                println!("If you are reading this, a placeholder for the clear function was fired\nThis shouldn\'t happen!");
            }),
            generate: Box::new(|| {
                println!("If you are reading this, a placeholder for the generate function was fired\nThis shouldn\'t happen!");
                "".into()
            })
        }
    }
}