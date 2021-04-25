use csscolorparser::{Color, ParseError};
use termion::color;

pub fn convert_hex_rgb(color: &str) -> Result<color::Rgb, ParseError> {
    let c = color.parse::<Color>()?;
    let (r, g, b, _) = c.rgba_u8();
    Ok(color::Rgb(r, g, b))
}
