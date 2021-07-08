mod entity;
mod image;
mod material;
mod math;

use rand::Rng;

use entity::{Entity, Sphere};
use image::{ppm, Camera, Image};
use material::{Dielectric, Lambertian, Metal};
use math::{Color, Point, Vector};

fn main() {
    let aspect_ratio = 3.0 / 2.0;

    let mut image = {
        let width = 1200;
        let height = ((width as f64) / aspect_ratio) as usize;
        Image::new(width, height)
    };

    let camera = {
        let look_from = Point::new(13.0, -3.0, 2.0);
        let look_at = (Point::new(0.0, 0.0, 0.0) - look_from).unit();
        let v_up = Vector::new(0.0, 0.0, 1.0);
        let v_fov = 20.0;
        Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio)
    };

    let mut world: Vec<Box<dyn Entity + Sync>> = vec![];
    world.push(Box::new(Sphere {
        center: Point::new(0.0, 0.0, -1000.0),
        radius: 1000.0,
        material: Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        },
    }));
    world.push(Box::new(Sphere {
        center: Point::new(-4.0, 0.0, 1.0),
        radius: 1.0,
        material: Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
    }));
    world.push(Box::new(Sphere {
        center: Point::new(0.0, 0.0, 1.0),
        radius: 1.0,
        material: Dielectric {
            albedo: Color::new(1.0, 1.0, 1.0),
            refraction_index: 1.5,
        },
    }));
    world.push(Box::new(Sphere {
        center: Point::new(4.0, 0.0, 1.0),
        radius: 1.0,
        material: Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    }));
    for a in -12..=12 {
        'positions: for b in -12..=12 {
            for i in -1..=1 {
                for j in -1..=1 {
                    if a == -4+i && b == 0+j {
                        continue 'positions;
                    }
                }
            }
            for i in -1..=1 {
                for j in -1..=1 {
                    if a == 0+i && b == 0+j {
                        continue 'positions;
                    }
                }
            }
            for i in -1..=1 {
                for j in -1..=1 {
                    if a == 4+i && b == 0+j {
                        continue 'positions;
                    }
                }
            }

            let center = Point::new(
                (a as f64) + rand::thread_rng().gen_range(-0.3..=0.3),
                (b as f64) + rand::thread_rng().gen_range(-0.3..=0.3),
                0.2,
            );
            let albedo = Color::new(
                rand::thread_rng().gen_range(0.0..=1.0),
                rand::thread_rng().gen_range(0.0..=1.0),
                rand::thread_rng().gen_range(0.0..=1.0),
            );
            let choose_material = rand::thread_rng().gen_range(0.0..1.0);

            if choose_material < 0.8 {
                world.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Lambertian {
                        albedo,
                    },
                }));
            } else if choose_material < 0.95 {
                world.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Metal {
                        albedo,
                        fuzz: rand::thread_rng().gen_range(0.0..0.5),
                    },
                }));
            } else {
                world.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Dielectric {
                        albedo,
                        refraction_index: 1.5,
                    },
                }));
            }
        }
    }

    image.render_threaded(&camera, &world, 50, 100, 8);

    ppm::output(&image);
}
