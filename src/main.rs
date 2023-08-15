extern crate gtk;
mod qrwindow;

use gtk::prelude::*;
use qrwindow::QrgenWindow;

fn main() {
    if let Err(err) = gtk::init() {
        eprintln!("Failed to initialize GTK: {}", err);
        return;
    }

    let my_window = QrgenWindow::new()
        .add_item("Text/URL")
        .add_item("Email")
        .add_item("Phone")
        .add_item("SMS")
        .add_item("Contact")
        .add_item("Geolocation")
        .add_item("WiFi")
        .add_item("Calendar Event")
        .add_item("Cryptocurrency")
        
        .build();

    my_window.show_all();
    gtk::main();
}
