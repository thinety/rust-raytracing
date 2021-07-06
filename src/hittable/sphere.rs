use crate::material::Material;
use crate::math::{Point, Ray, Vector};

use super::{HitRecord, Hittable};

pub struct Sphere<'a> {
    pub center: Point,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = Vector::dot(&ray.direction, &ray.direction);
        let half_b = Vector::dot(&ray.direction, &oc);
        let c = Vector::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();

            let mut root = (-half_b - sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                root = (-half_b + sqrt_discriminant) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            Some(HitRecord {
                t: root,
                normal: (ray.at(root) - self.center).unit(),
                material: self.material,
            })
        } else {
            None
        }
    }
}
