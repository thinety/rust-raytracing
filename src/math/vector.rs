use std::ops;

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        todo!()
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, _rhs: Self) {
        todo!()
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, _rhs: Self) {
        todo!()
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Self::Output {
        todo!()
    }
}
impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, _rhs: f64) {
        todo!()
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f64) -> Self::Output {
        todo!()
    }
}
impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, _rhs: f64) {
        todo!()
    }
}
