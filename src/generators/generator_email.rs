use gtk4::prelude::*;
use gtk4::{Entry};

use crate::generators::QrGenerator;

pub fn email_generator() -> QrGenerator {
    let mut my_gen = QrGenerator::new("Email");

    let inp_address = Entry::builder()
        .placeholder_text("Your email address")
        .build();

    let inp_subject = Entry::builder()
        .placeholder_text("Email Subject")
        .build();

    // TODO make the message field multi line
    let inp_message = Entry::builder()
        .placeholder_text("Email message")
        .build();

    my_gen.disp.append(&inp_address);
    my_gen.disp.append(&inp_subject);
    my_gen.disp.append(&inp_message);
    

    let inp_address_set = inp_address.clone();
    let inp_subject_set = inp_subject.clone();
    let inp_message_set = inp_message.clone();
    
    let inp_address_gen = inp_address.clone();
    let inp_subject_gen = inp_subject.clone();
    let inp_message_gen = inp_message.clone();
    
    
    my_gen.clear = Box::new(move || {
        inp_address_set.set_text("");
        inp_subject_set.set_text("");
        inp_message_set.set_text("");
    });

    my_gen.generate = Box::new(move || {
        let address = inp_address_gen.text();
        let subject = inp_subject_gen.text();
        let message = inp_message_gen.text();
        

        format!("MATMSG:TO:{};SUB:{};BODY:{};;", address, subject, message)
    });

    // Return generator
    my_gen
}