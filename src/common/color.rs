use crate::common::{Vec3, clamp};

/// alias for a RGB color with three color components
pub type Color = Vec3;

// max R,G, or B value for a PPM image
pub const MAX_RGB_COLOR: u8 = 255;

/// returns a new pixel color using multi-sample color computation
pub fn multi_sample_color(pixel_color: Color, samples_per_pixel: u32) -> Color {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // divide the color total by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // compute a translated [0..255] color value for each color's R,G,B
    Color::new(
        256.0 * clamp(r, 0.0, 0.999),
        256.0 * clamp(g, 0.0, 0.999),
        256.0 * clamp(b, 0.0, 0.999)
    )
}
