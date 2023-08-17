use gtk4::prelude::*;
use gtk4::{FileDialog, FileFilter, Window, Image, Box, Button, Orientation};
use gtk4::gdk_pixbuf::{Pixbuf, Colorspace};
use gtk4::gio::{Cancellable, ListStore};
use gdk4::{Texture};

use qrcode::QrCode;

const SIZE_MULT: u32 = 32;

pub fn show_code(code: &str) {
    let my_window = Window::new();
    my_window.set_title(Some("QR Code"));
    my_window.set_default_size(400, 400);

    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let image_pixbuf = generate_code(code);
    let image_texture = Texture::for_pixbuf(&image_pixbuf);

    let image_widget = Image::builder()
        .paintable(&image_texture)
        .hexpand(true)
        .vexpand(true)
        .build();
    let save_button = Button::builder()
        .label("Save QR Code")
        .margin_top(10)
        .build();

    let dialog_window = my_window.clone();
    save_button.connect_clicked(move |_| {
        // Make filedialog
        let dialog = FileDialog::new();
        dialog.set_title("Save QR Code");

        // Add filters
        let filter_list = ListStore::new::<FileFilter>();

        let png_filter = FileFilter::new();
        png_filter.add_mime_type("image/png");
        png_filter.set_name(Some("PNG"));
        filter_list.append(&png_filter);

        let jpg_filter = FileFilter::new();
        jpg_filter.add_mime_type("image/jpeg");
        jpg_filter.set_name(Some("JPG"));
        filter_list.append(&jpg_filter);

        dialog.set_filters(Some(&filter_list));
        dialog.set_default_filter(Some(&png_filter));

        // Save file
        let dialog_pixbuf = image_pixbuf.clone();

        dialog.save(Some(&dialog_window), Cancellable::NONE, move |file| {
            if let Ok(file) = file {
                if let Some(filepath) = file.path() {
                    let filepath_clone = filepath.clone();
                    let filename = filepath_clone.display().to_string();
                    // Guess the file type
                    let filetype;
                    let fileext = &filename[filename.len() - 4..];
                    if fileext == ".png" {
                        filetype = "png";
                    } else if fileext == ".jpg" || fileext == "jpeg" {
                        filetype = "jpeg";
                    } else {
                        panic!("TODO: Show error message here");
                    }

                    dialog_pixbuf.savev(filepath, filetype, &[]).expect("Failed to save QR code");
                    println!("QR image saved to \"{}\"", filename);
                }
            }
        });
    });

    container.append(&image_widget);
    container.append(&save_button);
    my_window.set_child(Some(&container));

    my_window.present();
}

fn generate_code(code: &str) -> Pixbuf {
    let qr_code = QrCode::new(code).unwrap();
    let qr_image = qr_code.render()
        .light_color('0')
        .dark_color('1')
        .build();

    let lines: Vec<&str> = qr_image.split('\n').collect();
    let img_size = lines.len() * SIZE_MULT as usize;

    let pixbuf = Pixbuf::new(
        Colorspace::Rgb,
        false,
        8,
        img_size as i32,
        img_size as i32,
    ).expect("Failed to create pixbuf");

    // Put pixels on pixbuf
    for cy in 0..lines.len() {
        for cx in 0..lines[cy].len() {
            if let Some(curr_line) = lines.get(cy) {
                if let Some(character) = curr_line.chars().nth(cx) {
                    // Get color
                    let color;
                    if character == '1' {
                        color = 0;  // Black
                    } else {
                        color = 255;// White
                    }

                    // Place pixels
                    for pix in 0..SIZE_MULT*SIZE_MULT {
                        let pixy = (pix as f64 / SIZE_MULT as f64).floor() as u32;

                        pixbuf.put_pixel((pix%32) as u32 + cx as u32*SIZE_MULT, pixy + cy as u32*SIZE_MULT, color, color, color, 255);
                    }

                } else {
                    panic!("Overflowed QR image string on X");
                }
            } else {
                panic!("Overflowed QR image string on Y");
            }
        }
    }

    pixbuf
}