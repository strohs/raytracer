use crate::common::Color;
use image::ColorType;
use std::path::Path;

pub fn write_file(
    file_path: impl AsRef<Path>,
    width: u32,
    height: u32,
    image: &[Color],
) -> image::ImageResult<()> {
    // extract the R,G,B color data from each Color struct in the image slice,
    // save it as a new slice of 8-bit R,G,B color values
    let mut rgbs: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);
    for r in 0..height {
        for c in 0..width {
            let idx = ((height - 1 - r) * width + c) as usize;
            let color = image[idx];
            rgbs.append(&mut color.as_array().map(|c| c as u8).to_vec());
        }
    }

    image::save_buffer(file_path, &rgbs, width, height, ColorType::Rgb8)
}
