use gtk::prelude::*;
use gtk::{Box, Entry, Orientation};

use crate::generators::QrGenerator;

pub fn url_box() -> QrGenerator {
    let my_gen = QrGenerator::new("Text/URL");

    let my_inp = Entry::builder()
        .placeholder_text("Text or URL")
        .build();
    my_gen.disp.add(&my_inp);

    // Return generator
    my_gen
}