use std::{fs::File, io::Read, path::Path};

pub use crate::color::*;

mod decode;
mod encode;

/// Rectangular images represented using RGB8.
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Image {
    /// Creates a new all black image
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Color::from([0u8, 0, 0, 0]); width * height],
        }
    }

    /// Consumes self and fills it with one color. Returns the filled image.
    pub fn fill(mut self, c: Color) -> Self {
        self.data = self.data.iter().map(|_| c).collect();
        return self;
    }

    /// Loads an image from a file.
    /// Currently only supports png file.
    pub fn from_file(file: impl AsRef<Path>) -> Result<Self, String> {
        decode::load_png(file)
    }

    /// Getter for image width
    pub fn width(&self) -> usize {
        self.width
    }
    /// Getter for image height
    pub fn height(&self) -> usize {
        self.height
    }
    /// Getter for the images raw pixel data
    pub fn data(&self) -> &Vec<Color> {
        &self.data
    }

    /// Returns the pixel at a given coordinate. Returns None if out of bounds.
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            Some(self.data[x + y * self.width])
        } else {
            None
        }
    }

    /// Returns the pixel at the given coordinate but uses signed integers for the coordinate.
    /// Returns none if out of bounds.
    pub fn signed_pixel_at(&self, x: isize, y: isize) -> Option<Color> {
        if x >= 0 && y >= 0 {
            self.pixel_at(x as usize, y as usize)
        } else {
            None
        }
    }

    /// Sets the pixel at a givent coordinate. Returns and error if out of bounds.
    pub fn set_pixel_at(&mut self, x: usize, y: usize, c: Color) -> Result<(), String> {
        if x < self.width && y < self.height {
            self.data[x + y * self.width] = c;
            Ok(())
        } else {
            Err(String::from("Coordinate is out of bounds"))
        }
    }

    /// Writes the image to a file
    /// Currently only supports png files.
    pub fn write_to_file(&self, file: impl AsRef<Path>) -> Result<(), String> {
        encode::write_file(self, file)
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Image {
            width: self.width,
            height: self.height,
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_parsing() {
        use std::str::FromStr;
        let a = Color::from_str("0xFFFFFF").unwrap();
        assert_eq!(Color::from([255u8, 255, 255]), a);
        let b = Color::from_str("#79a9d9").unwrap();
        assert_eq!(Color::from([121u8, 169, 217]), b);
        let c = Color::from_str("548941").unwrap();
        assert_eq!(Color::from([84u8, 137, 65]), c);
        let d = Color::from_str("#8aa83e9e").unwrap();
        assert_eq!(Color::from([138u8, 168, 62, 158]), d);
    }

    #[test]
    fn read_image_from_file() {
        let _img = Image::from_file("image/test.png").unwrap();
    }

    #[test]
    fn write_image_to_file() {
        let a = Image::from_file("image/test.png").unwrap();
        a.write_to_file("image/test.out.png").unwrap();
        let b = Image::from_file("image/test.out.png").unwrap();

        assert_eq!(a.width(), b.width());
        assert_eq!(a.height(), b.height());
    }
}
