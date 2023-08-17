use gtk4::prelude::*;
use gtk4::{Window, Image};
use gtk4::gdk_pixbuf::{Pixbuf, Colorspace};

use qrcode::QrCode;
use image::Luma;

pub fn show_code(code: &str) {
    let my_window = Window::new();
    my_window.set_title(Some("QR Code"));

    let qr_code = QrCode::new(code).unwrap();
    let qr_image = qr_code.render::<Luma<u8>>().build();

    let width = qr_image.width() as i32;
    let height = qr_image.height() as i32;

    let mut pixbuf = Pixbuf::new(
        Colorspace::Rgb,
        false,
        8,
        width,
        height,
    ).expect("Failed to create pixbuf from QR code image");

    for y in 0..height {
        for x in 0..width {
            let luma = qr_image.get_pixel(x as u32, y as u32)[0];
            let pixel = ((luma as u32) << 16) | ((luma as u32) << 8) | luma as u32;
            pixbuf.put_pixel(x as u32,
                             y as u32,
                             ((pixel >> 16) & 0xFF) as u8,
                             ((pixel >> 8) & 0xFF) as u8,
                             (pixel & 0xFF) as u8, 255);
        }
    }

    println!("x: {} | y: {}", width, height);

    let image_widget = Image::from_pixbuf(Some(&pixbuf));
    my_window.set_child(Some(&image_widget));

    my_window.show();
}
