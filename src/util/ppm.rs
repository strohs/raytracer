use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::LineWriter;
use crate::common::{Color, color};


pub fn write_file(file_path: &str, width: u32, height: u32, image: &[Color]) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = LineWriter::new(file);

    // write the PPM file "header"
    writer.write_all(b"P3\n")?;
    writer.write_all(format!("{} {}\n", width, height).as_bytes())?;
    writer.write_all(format!("{}\n", color::MAX_RGB_COLOR).as_bytes())?;

    // write the image data
    for color in image.iter() {
        writer.write_all(
            format!("{} {} {}\n", color.x() as u8, color.y() as u8, color.z() as u8).as_bytes()
        )?;
    }
    Ok(())
}
