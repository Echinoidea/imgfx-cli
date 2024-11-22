pub mod blend {

    use clap::builder::styling::RgbColor;
    use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

    use crate::utils::utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8};

    pub fn overlay(
        img: DynamicImage,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
        color: RgbColor,
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

            let (r, g, b) = (
                if lhs.0 < 128 {
                    ((lhs.0 as u16 * rhs.0 as u16) / 128) as u8
                } else {
                    255 - (((255 - lhs.0 as u16) * (255 - rhs.0 as u16)) / 128) as u8
                },
                if lhs.1 < 128 {
                    ((lhs.1 as u16 * rhs.1 as u16) / 128) as u8
                } else {
                    255 - (((255 - lhs.1 as u16) * (255 - rhs.1 as u16)) / 128) as u8
                },
                if lhs.2 < 128 {
                    ((lhs.2 as u16 * rhs.2 as u16) / 128) as u8
                } else {
                    255 - (((255 - lhs.2 as u16) * (255 - rhs.2 as u16)) / 128) as u8
                },
            );

            let a = in_pixel[3];

            *pixel = Rgba([r, g, b, a]);
        });

        output
    }

    pub fn screen(
        img: DynamicImage,
        lhs: Option<Vec<String>>,
        rhs: Option<Vec<String>>,
        color: RgbColor,
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

            let (r, g, b) = (
                255 - ((255 - lhs.0) as u16 * (255 - rhs.0) as u16 / 255) as u8,
                255 - ((255 - lhs.1) as u16 * (255 - rhs.1) as u16 / 255) as u8,
                255 - ((255 - lhs.2) as u16 * (255 - rhs.2) as u16 / 255) as u8,
            );

            let a = in_pixel[3];

            *pixel = Rgba([r, g, b, a]);
        });

        output
    }
}
