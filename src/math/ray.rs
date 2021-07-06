use super::{Point, Vector};

/// Direction vector is expected to be unit length.
/// Other code can (and probably will) rely on that.
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}
