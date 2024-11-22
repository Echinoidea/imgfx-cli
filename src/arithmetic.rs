pub mod arithmetic {
    use clap::builder::styling::RgbColor;
    use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

    use crate::utils::utils::{get_channel_by_name_rgb_color, get_channel_by_name_rgba_u8};

    pub fn add(
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

            let r = lhs.0.wrapping_add(rhs.0);
            let g = lhs.1.wrapping_add(rhs.1);
            let b = lhs.2.wrapping_add(rhs.2);
            let a = in_pixel[3];

            *pixel = Rgba([r, g, b, a]);
        });

        output
    }

    pub fn sub(
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

            let r = lhs.0 - rhs.0;
            let g = lhs.1 - rhs.1;
            let b = lhs.2 - rhs.2;
            let a = in_pixel[3];

            *pixel = Rgba([r, g, b, a]);
        });

        output
    }

    pub fn mult(
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

            let r = lhs.0.wrapping_mul(rhs.0);
            let g = lhs.1.wrapping_mul(rhs.1);
            let b = lhs.2.wrapping_mul(rhs.2);
            let a = in_pixel[3];

            *pixel = Rgba([r, g, b, a]);
        });

        output
    }

    pub fn div(
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

            let r = lhs.0 / rhs.0.max(1);
            let g = lhs.1 / rhs.1.max(1);
            let b = lhs.2 / rhs.2.max(1);
            let a = in_pixel[3];

            *pixel = Rgba([r, g, b, a]);
        });

        output
    }
}
