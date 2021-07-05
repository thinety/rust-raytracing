use crate::math::{Point3, Ray3, Vector3};

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    top_left_corner: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64, origin: Point3) -> Self {
        let viewport_width = viewport_height * aspect_ratio;

        let horizontal = Vector3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vector3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let focal = Vector3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

        let top_left_corner = origin + focal - horizontal / 2.0 - vertical / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            top_left_corner,
        }
    }

    pub fn get_ray(&self, h: f64, v: f64) -> Ray3 {
        let direction =
            (self.top_left_corner + h * self.horizontal + v * self.vertical) - self.origin;

        Ray3 {
            origin: self.origin,
            direction: direction.unit(),
        }
    }
}
