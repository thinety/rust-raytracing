use std::ops;

use super::Vector3;

#[derive(Clone, Copy)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Sub for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::AddAssign<Vector3> for Point3 {
    fn add_assign(&mut self, _rhs: Vector3) {
        todo!()
    }
}

impl ops::Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        self + -rhs
    }
}
impl ops::SubAssign<Vector3> for Point3 {
    fn sub_assign(&mut self, _rhs: Vector3) {
        todo!()
    }
}
