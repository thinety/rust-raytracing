mod sphere;

use crate::math::{Color, Ray, Vector};

pub use sphere::Sphere;

/// Normal vector is expected to be unit length.
/// Other code can (and probably will) rely on that.
pub struct HitRecord<'a> {
    pub ray: &'a Ray,
    pub t: f64,
    pub normal: Vector,
}

pub trait Hittable {
    fn hit<'a>(&self, ray: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;

    fn ray_color(&self, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // t_min starts at 0.0001 to fix shadow acne
        if let Some(hit_record) = self.hit(ray, 0.0001, f64::INFINITY) {
            let hit_point = hit_record.ray.at(hit_record.t);
            let target_point = hit_point + hit_record.normal + Vector::random_unit();

            let new_ray = Ray {
                origin: hit_point,
                direction: (target_point - hit_point).unit(),
            };

            return 0.5 * self.ray_color(&new_ray, depth - 1);
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
}

impl<T: Hittable + ?Sized> Hittable for Box<T> {
    fn hit<'a>(&self, ray: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        (**self).hit(ray, t_min, t_max)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit<'a>(&self, ray: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let mut t_closest_so_far = t_max;
        let mut option = None;

        for hittable in self.iter() {
            if let Some(hit_record) = hittable.hit(ray, t_min, t_closest_so_far) {
                t_closest_so_far = hit_record.t;
                option = Some(hit_record);
            }
        }

        option
    }
}
