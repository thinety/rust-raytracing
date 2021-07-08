mod entity;
mod image;
mod material;
mod math;

use entity::{Entity, Sphere};
use image::{ppm, Camera, Image};
use material::{Dielectric, Lambertian, Metal};
use math::{Color, Point, Vector};

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let mut image = {
        let width = 400;
        let height = ((width as f64) / aspect_ratio) as usize;
        Image::new(width, height)
    };

    let camera = {
        let look_from = Point::new(0.0, -1.0, 0.0);
        let look_at = (Point::new(0.0, 0.0, 0.0) - look_from).unit();
        let v_up = Vector::new(0.0, 0.0, 1.0);
        let v_fov = 90.0;
        Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio)
    };

    let world: Vec<Box<dyn Entity + Sync>> = vec![
        Box::new(Sphere {
            center: Point::new(0.0, 0.0, -100.5),
            radius: 100.0,
            material: Lambertian {
                albedo: Color::new(0.8, 0.8, 0.0),
            },
        }),
        Box::new(Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 0.5,
            material: Lambertian {
                albedo: Color::new(0.7, 0.3, 0.3),
            },
        }),
        Box::new(Sphere {
            center: Point::new(-1.0, 0.0, 0.0),
            radius: 0.5,
            material: Metal {
                albedo: Color::new(0.8, 0.8, 0.8),
                fuzz: 0.3,
            },
        }),
        Box::new(Sphere {
            center: Point::new(1.0, 0.0, 0.0),
            radius: 0.5,
            material: Dielectric {
                albedo: Color::new(1.0, 1.0, 1.0),
                refraction_index: 1.5,
            },
        }),
    ];

    image.render_threaded(&camera, &world, 50, 100, 8);

    ppm::output(&image);
}
