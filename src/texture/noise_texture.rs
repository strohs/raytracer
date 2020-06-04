use crate::texture::perlin::Perlin;
use crate::texture::Texture;
use crate::common::{Point3, Color};

/// Generates a "noisy" marble like texture, using Perlin Noise
#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {

    /// Creates a new Noise texture
    /// `perlin` is the Perlin noise generator to use
    /// `scale` is the amount to scale the input point by, in order to vary it more quickly
    pub fn new(noise: Perlin, scale: f64) -> Self {
        Self { noise, scale }
    }
}


impl Texture for NoiseTexture {

    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}