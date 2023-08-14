extern crate gtk;
mod qrwindow;

use gtk::prelude::*;
use qrwindow::QrgenWindow;

fn main() {
    if let Err(err) = gtk::init() {
        eprintln!("Failed to initialize GTK: {}", err);
        return;
    }

    let myWindow = QrgenWindow::new()
        .build();

    myWindow.show_all();
    gtk::main();
}
