use super::{HitRecord, Hittable};
use crate::math::{Point3, Ray3, Vector3};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit<'a>(&self, ray: &'a Ray3, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let oc = ray.origin - self.center;

        let a = Vector3::dot(&ray.direction, &ray.direction);
        let half_b = Vector3::dot(&ray.direction, &oc);
        let c = Vector3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        Some(HitRecord {
            ray,
            t: root,
            normal: (ray.at(root) - self.center).unit(),
        })
    }
}
