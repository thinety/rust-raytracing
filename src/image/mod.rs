mod camera;
pub mod ppm;

use rand::Rng;

use crate::hittable::Hittable;
use crate::math::Color;

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

                        color += world.ray_color(&ray, self.ray_depth);
                    }

                    let scale = 1.0 / (samples_per_pixel as f64);
                    color *= scale;
                } else {
                    let h = (j as f64) / ((self.width - 1) as f64);
                    let v = (i as f64) / ((self.height - 1) as f64);

                    let ray = camera.get_ray(h, v);

                    color += world.ray_color(&ray, self.ray_depth);
                }

                color.gamma_correct();

                self.pixels[i * self.width + j] = color;
            }
        }
        eprint!("done\n");
    }
}
