use gtk4::prelude::*;
use gtk4::{Entry};

use crate::generators::QrGenerator;

pub fn email_generator() -> QrGenerator {
    let mut my_gen = QrGenerator::new("Email");

    let my_inp = Entry::builder()
        .placeholder_text("Email")
        .build();
    my_gen.disp.append(&my_inp);

    my_gen.clear = Box::new(move || {
        my_inp.set_text("");
    });

    // Return generator
    my_gen
}