mod hittable;
mod image;
mod material;
mod math;

use hittable::{Hittable, Sphere};
use image::{ppm, Camera, Image};
use material::{
    black_body::{BlackBody, NormalBlackBody},
    Dielectric, Lambertian, Metal,
};
use math::{Color, Point, Vector};

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
        let look_from = Point::new(0.0, -1.0, 0.0);
        let look_to = (Point::new(0.0, 0.0, 0.0) - look_from).unit();
        let v_up = Vector::new(0.0, 0.0, 1.0);
        let v_fov = 90.0;
        Camera::new(look_from, look_to, v_up, v_fov, aspect_ratio)
    };

    // World
    let _perfect_absorber = BlackBody {
        color: Color::new(0.0, 0.0, 0.0),
    };
    let _perfect_emitter = BlackBody {
        color: Color::new(1.0, 1.0, 1.0),
    };
    let _normal_texture = NormalBlackBody {};

    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    };
    let material_left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Dielectric {
        albedo: Color::new(1.0, 1.0, 1.0),
        refraction_index: 1.5,
    };

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: Point::new(0.0, 0.0, -100.5),
            radius: 100.0,
            material: &material_ground,
        }),
        Box::new(Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 0.5,
            material: &material_center,
        }),
        Box::new(Sphere {
            center: Point::new(-1.0, 0.0, 0.0),
            radius: 0.5,
            material: &material_left,
        }),
        Box::new(Sphere {
            center: Point::new(1.0, 0.0, 0.0),
            radius: 0.5,
            material: &material_right,
        }),
    ];

    // Render
    image.render(&camera, &world);

    // Output
    ppm::output(&image);
}
