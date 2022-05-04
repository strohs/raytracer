use crate::common::Color;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::LineWriter;

pub const MAX_RGB_COLOR: u8 = 255;

/// writes the `image` data into a .ppm file
/// `file_path` is the path to the image file that will be written to
/// `width` the width of the image in pixels
/// `height` the height of the image in pixels
/// `image` the image data passed in as a slice of `Color`. The `Color` struct contains the actual RGB values
pub fn write_file(file_path: &str, width: u32, height: u32, image: &[Color]) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = LineWriter::new(file);

    // write the PPM file "header"
    writer.write_all(b"P3\n")?;
    writer.write_all(format!("{} {}\n", width, height).as_bytes())?;
    writer.write_all(format!("{}\n", MAX_RGB_COLOR).as_bytes())?;

    // write the image data in reverse row order (required by ppm image format)
    for r in 0..height {
        for c in 0..width {
            let idx = ((height - 1 - r) * width + c) as usize;
            let color = image[idx];
            writer.write_all(
                format!(
                    "{} {} {}\n",
                    color.x() as u8,
                    color.y() as u8,
                    color.z() as u8
                )
                .as_bytes(),
            )?;
        }
    }
    // for color in image.iter() {
    //     writer.write_all(
    //         format!("{} {} {}\n", color.x() as u8, color.y() as u8, color.z() as u8).as_bytes()
    //     )?;
    // }
    Ok(())
}
