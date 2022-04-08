use crate::common::{Color, Point3};
use crate::texture::Texture;

/// A solid color Texture
#[derive(Debug)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    /// Returns a `SolidColor` from the given `Color`
    pub fn from(color_value: Color) -> Self {
        Self { color_value }
    }

    /// Returns a `SolidColor` from the given RGB values
    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

// impl std::fmt::Debug for SolidColor {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("SolidColor")
//             .field("color_value", &self.color_value)
//             .finish()
//     }
// }
