use crate::texture::perlin::Perlin;
use crate::texture::Texture;
use crate::common::{Point3, Color};

/// Generates a marble like texture using Perlin Noise
/// `perlin` is the Perlin noise to use
/// `scale` is the amount to scale the input point by, to vary it more quickly
#[derive(Debug)]
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(noise: Perlin, scale: f64) -> Self {
        Self { noise, scale }
    }
}


impl Texture for Noise {

    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}