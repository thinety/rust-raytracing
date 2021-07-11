pub mod camera;
pub mod pixels;
pub mod ppm;

use rand::Rng;
use rayon;

use crate::entity::{Entity, EntityInterface};
use crate::material::{MaterialInterface, ScatterRecord};
use crate::math::{Color, Ray};

use camera::Camera;

pub struct Image {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];
        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn _render(&mut self, camera: &Camera, world: &[Entity], ray_depth: usize) {
        for i in 0..self.height {
            for j in 0..self.width {
                let color = &mut self.pixels[i * self.width + j];

                let i = (i as f64) / ((self.height - 1) as f64);
                let j = (j as f64) / ((self.width - 1) as f64);

                let ray = camera.get_ray(i, j);

                *color = ray_color(world, &ray, ray_depth);

                color.gamma_correct();
            }
        }
    }

    pub fn _render_antialiasing(
        &mut self,
        camera: &Camera,
        world: &[Entity],
        ray_depth: usize,
        samples_per_pixel: usize,
    ) {
        for i in 0..self.height {
            for j in 0..self.width {
                let color = &mut self.pixels[i * self.width + j];

                *color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..samples_per_pixel {
                    let i = ((i as f64) + rand::thread_rng().gen_range(-0.5..=0.5))
                        / ((self.height - 1) as f64);
                    let j = ((j as f64) + rand::thread_rng().gen_range(-0.5..=0.5))
                        / ((self.width - 1) as f64);

                    let ray = camera.get_ray(i, j);

                    *color += ray_color(world, &ray, ray_depth);
                }

                *color *= 1.0 / (samples_per_pixel as f64);

                color.gamma_correct();
            }
        }
    }

    pub fn render_threaded(
        &mut self,
        camera: &Camera,
        world: &[Entity],
        ray_depth: usize,
        samples_per_pixel_per_thread: usize,
        thread_num: usize,
    ) {
        self.pixels = render_threaded(
            self.width,
            self.height,
            camera,
            world,
            ray_depth,
            samples_per_pixel_per_thread,
            thread_num,
        );
    }
}

fn render_threaded(
    width: usize,
    height: usize,
    camera: &Camera,
    world: &[Entity],
    ray_depth: usize,
    samples_per_pixel_per_thread: usize,
    thread_num: usize,
) -> Vec<Color> {
    if thread_num == 1 {
        let mut pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];

        for i in 0..height {
            for j in 0..width {
                let color = &mut pixels[i * width + j];

                for _ in 0..samples_per_pixel_per_thread {
                    let i = ((i as f64) + rand::thread_rng().gen_range(-0.5..=0.5))
                        / ((height - 1) as f64);
                    let j = ((j as f64) + rand::thread_rng().gen_range(-0.5..=0.5))
                        / ((width - 1) as f64);

                    let ray = camera.get_ray(i, j);

                    *color += ray_color(world, &ray, ray_depth);
                }

                *color *= 1.0 / (samples_per_pixel_per_thread as f64);

                color.gamma_correct();
            }
        }

        pixels
    } else {
        let operation = || {
            render_threaded(
                width,
                height,
                camera,
                world,
                ray_depth,
                samples_per_pixel_per_thread,
                thread_num / 2,
            )
        };
        let (mut pixels_a, pixels_b) = rayon::join(operation, operation);

        for i in 0..height {
            for j in 0..width {
                let color = &mut pixels_a[i * width + j];

                *color += pixels_b[i * width + j];
                *color *= 1.0 / 2.0;
            }
        }

        pixels_a
    }
}

fn ray_color(world: &[Entity], ray: &Ray, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut t_closest_so_far = f64::INFINITY;
    let mut hit = None;
    for entity in world {
        // t_min starts at 0.0001 to fix shadow acne
        if let Some(hit_record) = entity.hit(ray, 0.0001, t_closest_so_far) {
            t_closest_so_far = hit_record.t;
            hit = Some(hit_record);
        }
    }

    if let Some(hit_record) = hit {
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
        let t = 0.5 * (1.0 + ray.direction.z);
        let blue = Color::new(0.5, 0.7, 1.0);
        let white = Color::new(1.0, 1.0, 1.0);
        t * blue + (1.0 - t) * white
    }
}
