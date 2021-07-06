use super::Image;

pub fn output(image: &Image) {
    eprint!("Outputting image (ppm format)... ");
    print!("P3\n{} {}\n255\n", image.width, image.height);
    for pixel in image.pixels.iter() {
        print!(
            "{} {} {}\n",
            (255.0 * pixel.r).round() as u8,
            (255.0 * pixel.g).round() as u8,
            (255.0 * pixel.b).round() as u8,
        );
    }
    eprint!("done\n");
}
