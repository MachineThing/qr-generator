use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Application, Box, ListBox, Label, Stack, Orientation, GestureClick};

use crate::generators;
use generators::QrGenerator;

pub struct QrgenWindow {
    my_window: ApplicationWindow,
    my_box: Box,
    my_stack: Stack,
    my_list: ListBox
}

impl QrgenWindow {
    pub fn new(app: &Application) -> Self {
        let new_window = ApplicationWindow::builder()
            .application(app)
            .title("QR Code Generator")
            .default_width(750)
            .default_height(500)
            .build();

        // Return self
        Self {
            my_window: new_window,
            my_box: Box::new(Orientation::Horizontal, 0),
            my_stack: Stack::new(),
            my_list: ListBox::new()
        }
    }

    pub fn add_item(self, generator: QrGenerator) -> Self {
        if let Some(child_widget) = generator.row.child() {
            if let Some(child_label) = child_widget.downcast_ref::<Label>() {
                // Get text
                let text = child_label.label().to_string();

                // Append children
                self.my_list.append(&generator.row);
                self.my_stack.add_named(&generator.disp, Some(&text));

                // Handle Clicking
                let gesture = GestureClick::new();
                let stack_clone = self.my_stack.clone();
                gesture.set_button(gtk4::gdk::ffi::GDK_BUTTON_PRIMARY as u32);
                gesture.connect_pressed(move |_,_,_,_| {
                    println!("Selected item: {}", &text);
                    stack_clone.set_visible_child_name(&text);
                });
            
                generator.row.add_controller(gesture);
            } else {
                panic!("Child is not a label");
            }
        } else {
            panic!("No children");
        }

        // Return self
        self
    }

    pub fn build(self) -> ApplicationWindow {
        self.my_box.append(&self.my_list);
        self.my_box.append(&self.my_stack);
        self.my_window.set_child(Some(&self.my_box));

        // Return QrgenWindow
        self.my_window
    }
}