use crate::math::{Point, Ray, Vector};

pub struct Camera {
    look_from: Point,
    look_at: Vector,
    vertical: Vector,
    horizontal: Vector,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Vector,
        v_up: Vector,
        v_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let u = Vector::cross(&look_at, &v_up).unit();
        let v = Vector::cross(&look_at, &u).unit();
        // u, v and w = look_at together form an
        // orthonormal base (assuming look_at is unit)

        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        Self {
            look_from,
            look_at,
            horizontal: viewport_width * u,
            vertical: viewport_height * v,
        }
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        let origin = self.look_from;
        let direction =
            (self.look_at + (i - 0.5) * self.vertical + (j - 0.5) * self.horizontal).unit();

        Ray { origin, direction }
    }
}
