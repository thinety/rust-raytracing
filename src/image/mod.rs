mod camera;
pub mod ppm;

use rand::Rng;

use crate::hittable::Hittable;
use crate::material::ScatterRecord;
use crate::math::{Color, Ray};

pub use camera::Camera;

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
    antialiasing: Option<u32>,
    ray_depth: u32,
}

impl Image {
    pub fn new(width: usize, height: usize, antialiasing: Option<u32>, ray_depth: u32) -> Self {
        let pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];
        Self {
            width,
            height,
            pixels,
            antialiasing,
            ray_depth,
        }
    }

    pub fn render<World: Hittable>(&mut self, camera: &Camera, world: &World) {
        eprint!("Rendering image... ");
        for i in 0..self.height {
            for j in 0..self.width {
                let mut color = Color::new(0.0, 0.0, 0.0);

                if let Some(samples_per_pixel) = self.antialiasing {
                    for _ in 0..samples_per_pixel {
                        let h = ((j as f64) + rand::thread_rng().gen_range(0.0..1.0))
                            / ((self.width - 1) as f64);
                        let v = ((i as f64) + rand::thread_rng().gen_range(0.0..1.0))
                            / ((self.height - 1) as f64);

                        let ray = camera.get_ray(h, v);

                        color += ray_color(world, &ray, self.ray_depth);
                    }

                    let scale = 1.0 / (samples_per_pixel as f64);
                    color *= scale;
                } else {
                    let h = (j as f64) / ((self.width - 1) as f64);
                    let v = (i as f64) / ((self.height - 1) as f64);

                    let ray = camera.get_ray(h, v);

                    color += ray_color(world, &ray, self.ray_depth);
                }

                color.gamma_correct();

                self.pixels[i * self.width + j] = color;
            }
        }
        eprint!("done\n");
    }
}

fn ray_color<World: Hittable>(world: &World, ray: &Ray, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // t_min starts at 0.0001 to fix shadow acne
    if let Some(hit_record) = world.hit(ray, 0.0001, f64::INFINITY) {
        let hit_direction = ray.direction;
        let hit_normal = hit_record.normal;
        let hit_material = hit_record.material;

        match hit_material.scatter(hit_direction, hit_normal) {
            ScatterRecord::Ideal { color } => color,
            ScatterRecord::NonIdeal {
                attenuation,
                direction,
            } => {
                let hit_point = ray.at(hit_record.t);
                let scattered_ray = Ray {
                    origin: hit_point,
                    direction,
                };
                attenuation * ray_color(world, &scattered_ray, depth - 1)
            }
        }
    } else {
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
}
