use crate::common::{clamp, Color, Point3};
use crate::texture::Texture;
use image::{DynamicImage, GenericImageView};

const BYTES_PER_PIXEL: u32 = 3;
const COLOR_SCALE: f64 = 1.0 / 255.0;

/// Enables in image to be texture mapped onto a Hittable
/// To test this, assign it to a sphere, and then temporarily cripple the ray_color() function
/// in `Renderer` to just return attenuation
#[derive(Debug, Default)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl ImageTexture {
    pub fn from(file_name: &str) -> Self {
        let img: DynamicImage = image::open(file_name)
            .unwrap_or_else(|_| panic!("could not load image at {}", file_name));
        let (width, height) = img.dimensions();
        let data: Vec<u8> = img.into_rgb().into_vec();
        let bytes_per_scanline = width * BYTES_PER_PIXEL;

        Self {
            data,
            width,
            height,
            bytes_per_scanline,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // if no texture data, return solid cyan as a debugging aid
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        // clamp texture coordinates to [0,1] x [1,0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); //flip v to image coordinates

        let i = {
            let i = (u * self.width as f64) as usize;
            // Clamp integer mapping, since actual coordinates should be less than 1.0
            if i >= self.width as usize {
                self.width as usize - 1
            } else {
                i
            }
        };
        let j = {
            let j = (v * self.height as f64) as usize;
            // Clamp integer mapping, since actual coordinates should be less than 1.0
            if j >= self.height as usize {
                self.height as usize - 1
            } else {
                j
            }
        };

        let idx = j * self.bytes_per_scanline as usize + i * BYTES_PER_PIXEL as usize;
        let pixel = &self.data[idx..idx + 3];

        Color::new(
            COLOR_SCALE * pixel[0] as f64,
            COLOR_SCALE * pixel[1] as f64,
            COLOR_SCALE * pixel[2] as f64,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::texture::ImageTexture;

    #[test]
    fn load_an_image() {
        let image_tex = ImageTexture::from("./earthmap.jpg");
        println!(
            "loaded earthmap.jpg.  image_tex {} {} {}",
            image_tex.width, image_tex.height, image_tex.bytes_per_scanline
        );
    }

    #[test]
    fn has_default_impl() {
        let image_tex = ImageTexture::default();
        assert_eq!(image_tex.width, 0);
        assert_eq!(image_tex.height, 0);
        assert_eq!(image_tex.bytes_per_scanline, 0);
        assert!(image_tex.data.is_empty());
    }
}
