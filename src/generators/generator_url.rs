use gtk4::prelude::*;
use gtk4::{Entry};

use crate::generators::QrGenerator;

pub fn url_generator() -> QrGenerator {
    let my_gen = QrGenerator::new("Text/URL");

    let my_inp = Entry::builder()
        .placeholder_text("Text or URL")
        .build();
    my_gen.disp.append(&my_inp);

    // Return generator
    my_gen
}