#![feature(new_uninit)]

mod image;

use std::mem::MaybeUninit;

use image::{Color, Pixel};

fn main() {
    // Image
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Pixel data
    let pixels: Box<MaybeUninit<[Pixel; IMAGE_WIDTH * IMAGE_HEIGHT]>> = Box::new_uninit();

    let mut pixels: Box<[[MaybeUninit<Pixel>; IMAGE_WIDTH]; IMAGE_HEIGHT]> =
        unsafe { Box::from_raw(Box::into_raw(pixels) as _) };

    eprint!("\nCalculating pixel data\n");
    for (i, line) in pixels.iter_mut().enumerate() {
        for (j, pixel) in line.iter_mut().enumerate() {
            *pixel = MaybeUninit::new(Pixel {
                color: Color {
                    r: (i as f64) / ((IMAGE_HEIGHT - 1) as f64),
                    g: (j as f64) / ((IMAGE_WIDTH - 1) as f64),
                    b: 0.25,
                },
            });
            eprint!(
                "\rPixels remaining: {:4} x {:4}",
                IMAGE_HEIGHT - i - 1,
                IMAGE_WIDTH - j - 1,
            );
        }
    }
    eprint!("\nDone\n");

    let pixels: Box<[Pixel; IMAGE_WIDTH * IMAGE_HEIGHT]> =
        unsafe { Box::from_raw(Box::into_raw(pixels) as _) };

    // Render (ppm format)
    eprint!("\nRendering image\n");
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for (i, pixel) in pixels.iter().enumerate() {
        print!("{}\n", pixel);
        eprint!(
            "\rPixels remaining: {:7}",
            IMAGE_WIDTH * IMAGE_HEIGHT - i - 1
        )
    }
    eprint!("\nDone\n");
}
