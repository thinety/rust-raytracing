mod sphere;

use crate::math::{Ray3, Vector3};
pub use sphere::Sphere;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub ray: &'a Ray3,
    pub t: f64,
    pub normal: Vector3,
}

pub trait Hittable {
    fn hit<'a>(&self, ray: &'a Ray3, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;
}

impl<T: Hittable + ?Sized> Hittable for Box<T> {
    fn hit<'a>(&self, ray: &'a Ray3, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        (**self).hit(ray, t_min, t_max)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit<'a>(&self, ray: &'a Ray3, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
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
