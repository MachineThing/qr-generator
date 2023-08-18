extern crate gtk4;

pub mod qrwindow;
pub mod showcode;
pub mod generators;

use gtk4::prelude::*;
use gtk4::{glib, Application};
use qrwindow::QrgenWindow;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("net.masonfisher.qrgen")
        .build();
    
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let my_window = QrgenWindow::new(app)
        .add_item(generators::url_generator())
        .add_item(generators::email_generator())
        //.add_item("Phone")
        //.add_item("SMS")
        //.add_item("Contact")
        //.add_item("Geolocation")
        .add_item(generators::wifi_generator())
        //.add_item("Calendar Event")
        //.add_item("Cryptocurrency")
        
        .build();

    my_window.present();
}