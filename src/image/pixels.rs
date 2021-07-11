use super::Image;

pub fn output(image: &Image, buffer: &mut [u8]) {
    for (i, pixel) in image.pixels.iter().enumerate() {
        let i = 4 * i;

        // Since Rust 1.45, this performs a *saturating cast*
        // (https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#fixing-unsoundness-in-casts).
        // So the only case that could cause problems (r, g or b equal to 1.0) is taken care of automatically.
        buffer[i + 0] = (256.0 * pixel.r) as u8;
        buffer[i + 1] = (256.0 * pixel.g) as u8;
        buffer[i + 2] = (256.0 * pixel.b) as u8;

        buffer[i + 3] = 255;
    }
}
