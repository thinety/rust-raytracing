use super::Image;

pub fn _output(image: &Image) {
    eprint!("Outputting image (ppm format)... ");
    print!("P3\n{} {}\n255\n", image.width, image.height);
    for pixel in &image.pixels {
        // Since Rust 1.45, this performs a *saturating cast*
        // (https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#fixing-unsoundness-in-casts).
        // So the only case that could cause problems (r, g or b equal to 1.0) is taken care of automatically.
        print!(
            "{} {} {}\n",
            (256.0 * pixel.r) as u8,
            (256.0 * pixel.g) as u8,
            (256.0 * pixel.b) as u8,
        );
    }
    eprint!("done\n");
}
