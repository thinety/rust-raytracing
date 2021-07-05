#![feature(new_uninit)]

mod image;
mod math;

use std::mem::MaybeUninit;

use image::{Color, Pixel};
use math::{Point3, Ray3, Vector3};

const SPHERE_CENTER: Point3 = Point3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};
const SPHERE_RADIUS: f64 = 0.5;
fn hit_sphere(ray: &Ray3) -> bool {
    let oc = ray.origin - SPHERE_CENTER;

    let a = Vector3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * Vector3::dot(&oc, &ray.direction);
    let c = Vector3::dot(&oc, &oc) - SPHERE_RADIUS * SPHERE_RADIUS;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn ray_color(ray: &Ray3) -> Color {
    if hit_sphere(ray) {
        return Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        };
    }

    let t = 0.5 * (1.0 - ray.direction.y);
    let blue = Color {
        r: 0.5,
        g: 0.7,
        b: 1.0,
    };
    let white = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    t * blue + (1.0 - t) * white
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vector3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vector3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let focal = Vector3 {
        x: 0.0,
        y: 0.0,
        z: focal_length,
    };
    let top_left_corner = origin + focal - horizontal / 2.0 - vertical / 2.0;

    // Pixel data
    let pixels: Box<MaybeUninit<[Pixel; IMAGE_WIDTH * IMAGE_HEIGHT]>> = Box::new_uninit();

    let mut pixels: Box<[[MaybeUninit<Pixel>; IMAGE_WIDTH]; IMAGE_HEIGHT]> =
        unsafe { Box::from_raw(Box::into_raw(pixels) as _) };

    eprint!("\nCalculating pixel data\n");
    for (i, line) in pixels.iter_mut().enumerate() {
        for (j, pixel) in line.iter_mut().enumerate() {
            let h = (j as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (i as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let ray = Ray3 {
                origin,
                direction: ((top_left_corner + h * horizontal + v * vertical) - origin).unit(),
            };

            let color = ray_color(&ray);

            *pixel = MaybeUninit::new(Pixel { color });
        }
    }
    eprint!("\nDone\n");

    let pixels: Box<[Pixel; IMAGE_WIDTH * IMAGE_HEIGHT]> =
        unsafe { Box::from_raw(Box::into_raw(pixels) as _) };

    // Render (ppm format)
    eprint!("\nRendering image\n");
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for pixel in pixels.iter() {
        print!("{}\n", pixel);
    }
    eprint!("\nDone\n");
}
