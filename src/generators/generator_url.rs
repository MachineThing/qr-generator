use gtk4::prelude::*;
use gtk4::{Entry, glib::GString};

use crate::generators::QrGenerator;

pub fn url_generator() -> QrGenerator {
    let mut my_gen = QrGenerator::new("Text/URL");

    let my_inp = Entry::builder()
        .placeholder_text("Text or URL")
        .build();
    my_gen.disp.append(&my_inp);

    let inp_set = my_inp.clone();
    let inp_gen = my_inp.clone();
    
    my_gen.clear = Box::new(move || {
        inp_set.set_text("");
    });

    my_gen.generate = Box::new(move || {
        let text: GString = inp_gen.text();
        text.to_string()
    });

    // Return generator
    my_gen
}