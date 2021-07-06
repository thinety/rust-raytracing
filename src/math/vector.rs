use rand::Rng;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn unit(self) -> Self {
        let length_squared = Self::dot(&self, &self);
        let length = length_squared.sqrt();
        let inverse_length = 1.0 / length;
        inverse_length * self
    }

    pub fn random_unit() -> Self {
        Self::new(
            rand::thread_rng().gen_range(-1.0..=1.0),
            rand::thread_rng().gen_range(-1.0..=1.0),
            rand::thread_rng().gen_range(-1.0..=1.0),
        )
        .unit()
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}
