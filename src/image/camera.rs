use crate::math::{Point, Ray, Vector};

pub struct Camera {
    origin: Point,
    horizontal: Vector,
    vertical: Vector,
    top_left_corner: Point,
}

impl Camera {
    pub fn new(
        viewport_width: f64,
        viewport_height: f64,
        focal_length: f64,
        origin: Point,
    ) -> Self {
        let horizontal = Vector::new(viewport_width, 0.0, 0.0);
        let vertical = Vector::new(0.0, viewport_height, 0.0);
        let focal = Vector::new(0.0, 0.0, focal_length);

        let top_left_corner = origin + focal - horizontal / 2.0 - vertical / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            top_left_corner,
        }
    }

    pub fn get_ray(&self, h: f64, v: f64) -> Ray {
        let direction =
            (self.top_left_corner + h * self.horizontal + v * self.vertical) - self.origin;

        Ray {
            origin: self.origin,
            direction: direction.unit(),
        }
    }
}
