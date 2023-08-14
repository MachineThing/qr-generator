extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Window, WindowType};

fn main() {
    // Initialize GTK
    if let Err(err) = gtk::init() {
        eprintln!("Failed to initialize GTK: {}", err);
        return;
    }

    // Create a new top-level window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("QR Code Generator");
    window.set_default_size(750, 500);

    // Create a label widget with the text
    let label = Label::new(Some("Hello, GTK from Rust!"));

    // Add the label to the window
    window.add(&label);

    // Connect the "delete-event" signal to terminate the application
    window.connect_delete_event(|_, _| {
        // Terminate the GTK main loop
        gtk::main_quit();
        false.into()
    });

    // Show all the widgets
    window.show_all();

    // Run the GTK main loop
    gtk::main();
}
