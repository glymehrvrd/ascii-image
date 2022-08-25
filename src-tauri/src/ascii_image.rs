use image::{
    imageops::FilterType, io::Reader as ImageReader, DynamicImage, GenericImageView, Pixel,
};
use std::io::{BufRead, Seek};

pub struct AsciiImage {
    img: DynamicImage,
}

impl AsciiImage {
    pub fn new<T: BufRead + Seek>(r: T) -> Result<Self, image::ImageError> {
        let img = ImageReader::new(r).with_guessed_format()?.decode()?;
        Ok(AsciiImage { img })
    }

    pub fn to_ascii(&self, width: u32, height: u32) -> String {
        const ASCII_CHAR: &str =
            r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,\"^`'. "#;
        let img = self
            .img
            .resize(width, height, FilterType::Nearest)
            .grayscale();
        let mut result = String::new();
        result.reserve((width * height) as usize);
        for (x, y, color) in img.pixels() {
            if x == 0 && y != 0 {
                result.push('\n');
            }
            let s = color.to_luma().0[0];
            let idx = (((ASCII_CHAR.len() - 1) as f64 / 255 as f64) * s as f64).round() as usize;
            result.push(ASCII_CHAR.chars().nth(idx).unwrap());
        }
        return result;
    }
}
