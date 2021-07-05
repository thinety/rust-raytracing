use std::{fmt, ops};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub color: Color,
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
impl ops::AddAssign for Color {
    fn add_assign(&mut self, _rhs: Self) {
        todo!()
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Self::Output {
        todo!()
    }
}
impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, _rhs: f64) {
        todo!()
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.0 * self.color.r) as u32,
            (255.0 * self.color.g) as u32,
            (255.0 * self.color.b) as u32,
        )
    }
}
