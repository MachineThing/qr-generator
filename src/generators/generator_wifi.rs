use gtk4::prelude::*;
use gtk4::{Entry, CheckButton, glib::GString};

use crate::generators::QrGenerator;

pub fn wifi_generator() -> QrGenerator {
    let mut my_gen = QrGenerator::new("WiFi");

    // Widgets
    let inp_ssid = Entry::builder()
        .placeholder_text("Network SSID")
        .build();
    
    let inp_pass = Entry::builder()
        .placeholder_text("Password")
        .build();
    inp_pass.set_visibility(false);

    let inp_hide = CheckButton::builder()
        .label("Hidden network")
        .build();
    
    let inp_encr_none = CheckButton::builder()
        .label("None")
        .build();

    let inp_encr_wpa = CheckButton::builder()
        .label("WPA/WPA2/WPA3")
        .build();
    inp_encr_wpa.set_group(Some(&inp_encr_none));
    inp_encr_wpa.set_active(true);
    
    let inp_encr_wep = CheckButton::builder()
        .label("WEP")
        .build();
    inp_encr_wep.set_group(Some(&inp_encr_none));

    // Widget events

    let inp_pass_check = inp_pass.clone();

    inp_encr_none.connect_toggled(move |check| {
        inp_pass_check.set_sensitive(!check.is_active());
    });
    
    // Apply widgets
    my_gen.disp.append(&inp_ssid);
    my_gen.disp.append(&inp_pass);
    my_gen.disp.append(&inp_hide);

    my_gen.disp.append(&inp_encr_none);
    my_gen.disp.append(&inp_encr_wpa);
    my_gen.disp.append(&inp_encr_wep);

    // Clear/Generate
    let inp_ssid_set = inp_ssid.clone();
    let inp_ssid_gen = inp_ssid.clone();
    
    my_gen.clear = Box::new(move || {
        inp_ssid_set.set_text("");
    });

    my_gen.generate = Box::new(move || {
        let text: GString = inp_ssid_gen.text();
        text.to_string()
    });

    // Return generator
    my_gen
}