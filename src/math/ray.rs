use super::{Point3, Vector3};

#[derive(Clone, Copy)]
pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray3 {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
