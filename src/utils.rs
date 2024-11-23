use clap::builder::styling::RgbColor;
use image::Rgba;

pub fn get_channel_by_name_rgb_color(name: &str, color: &RgbColor) -> u8 {
    match name {
        "r" => color.r(),
        "g" => color.g(),
        "b" => color.b(),
        _ => 0,
    }
}

pub fn get_channel_by_name_rgba_u8(name: &str, color: &Rgba<u8>) -> u8 {
    match name {
        "r" => color[0],
        "g" => color[1],
        "b" => color[2],
        _ => 0,
    }
}

pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}
