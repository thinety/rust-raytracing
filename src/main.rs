mod hittable;
mod image;
mod math;

use hittable::{Hittable, Sphere};
use image::{ppm, Camera, Image};
use math::Point;

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    // Image
    let mut image = {
        let width = 400;
        let height = ((width as f64) / aspect_ratio) as usize;
        let antialiasing = Some(100);
        let ray_depth = 50;
        Image::new(width, height, antialiasing, ray_depth)
    };

    // Camera
    let camera = {
        let viewport_width = 4.0;
        let viewport_height = viewport_width / aspect_ratio;
        let focal_length = 1.0;
        let origin = Point::new(0.0, 0.0, 0.0);
        Camera::new(viewport_width, viewport_height, focal_length, origin)
    };

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Point::new(0.0, 0.0, 1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Point::new(0.0, 100.5, 1.0),
            radius: 100.0,
        }),
    ];

    // Render
    image.render(&camera, &world);

    // Output
    ppm::output(&image);
}
