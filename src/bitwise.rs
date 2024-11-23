use clap::builder::styling::RgbColor;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

use crate::utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8};

pub fn or(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
    negate: bool,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.g(), color.b()),
    };

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let (r, g, b) = match negate {
            true => (!(lhs.0 | rhs.0), !(lhs.1 | rhs.1), !(lhs.2 | rhs.2)),
            false => ((lhs.0 | rhs.0), (lhs.1 | rhs.1), (lhs.2 | rhs.2)),
        };

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn and(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
    negate: bool,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.g(), color.b()),
    };

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let (r, g, b) = match negate {
            true => (!(lhs.0 & rhs.0), !(lhs.1 & rhs.1), !(lhs.2 & rhs.2)),
            false => ((lhs.0 & rhs.0), (lhs.1 & rhs.1), (lhs.2 & rhs.2)),
        };

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub fn xor(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
    negate: bool,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.g(), color.b()),
    };

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let (r, g, b) = match negate {
            true => (!(lhs.0 ^ rhs.0), !(lhs.1 ^ rhs.1), !(lhs.2 ^ rhs.2)),
            false => ((lhs.0 ^ rhs.0), (lhs.1 ^ rhs.1), (lhs.2 ^ rhs.2)),
        };

        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

pub enum BitshiftDirection {
    LEFT,
    RIGHT,
}

pub fn bitshift(img: DynamicImage, direction: BitshiftDirection, bits: u8, raw: bool) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let (r, g, b, a) = match direction {
            BitshiftDirection::LEFT => {
                if raw {
                    (
                        ((in_pixel[0] as u16) << bits) as u8,
                        ((in_pixel[1] as u16) << bits) as u8,
                        ((in_pixel[2] as u16) << bits) as u8,
                        in_pixel[3],
                    )
                } else {
                    (
                        ((in_pixel[0] as u16) << bits).min(255) as u8,
                        ((in_pixel[1] as u16) << bits).min(255) as u8,
                        ((in_pixel[2] as u16) << bits).min(255) as u8,
                        in_pixel[3],
                    )
                }
            }
            BitshiftDirection::RIGHT => {
                if raw {
                    (
                        (in_pixel[0] >> bits),
                        (in_pixel[1] >> bits),
                        (in_pixel[2] >> bits),
                        in_pixel[3],
                    )
                } else {
                    (
                        (in_pixel[0].wrapping_shr(bits.into())),
                        (in_pixel[1].wrapping_shr(bits.into())),
                        (in_pixel[2].wrapping_shr(bits.into())),
                        in_pixel[3],
                    )
                }
            }
        };
        *pixel = Rgba([r, g, b, a]);
    });

    output
}
