use gtk::prelude::*;
use gtk::{Box, Orientation, Label, ListBoxRow};

mod generator_url;
pub use generator_url::url_box;

pub struct QrGenerator {
    pub disp: Box,
    pub row: ListBoxRow,
}

impl QrGenerator {
    pub fn new(name: &str) -> Self {
        // disp
        let my_box = Box::builder()
            .orientation(Orientation::Vertical)
            .expand(true)
            .margin(10)
            .build();

        // row
        let my_row = ListBoxRow::new();
        let my_label = Label::builder()
            .label(name)
            .margin(10)
            .build();

        my_row.add(&my_label);

        // Return self
        Self {
            disp: my_box,
            row: my_row
        }
    }
}