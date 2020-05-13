
pub trait RGBStringify {

    /// returns three RGB color values in the range 0 to 255 separated by a space
    fn to_rgb_string(&self) -> String;
}