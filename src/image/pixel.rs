use std::fmt;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct Pixel {
    pub color: Color,
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
