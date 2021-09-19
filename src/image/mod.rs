use std::{fs::File, io::Read, path::Path};

pub use crate::color::*;

pub mod filter;

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

    /// Sets the pixel at a given coordinate. Returns and error if out of bounds.
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

    pub fn kernel_filter_pixel(&self, x: usize, y: usize, kernel: &filter::Kernel) -> Color {
        use crate::vector::Vector;
        use std::convert::TryInto;

        let mut color_acc = Vector::<4>::new();
        let mut value_acc = 0.0;

        for (offset, value) in kernel.pairs() {
            let x = x as isize + offset.0;
            let y = y as isize + offset.1;

            let color = self.pixel_at(x as usize, y as usize);

            match color {
                Some(c) => {
                    color_acc += c.as_vector() * value;
                    value_acc += value;
                },

                None => continue,
            }
        }

        return (color_acc * (1.0 / value_acc)).try_into().unwrap();
    }

    pub fn kernel_filter(&self, kernel: &filter::Kernel) -> Image {
        let mut filtered = self.clone();

        for x in 0..filtered.width() {
            for y in 0..filtered.height() {
                filtered.set_pixel_at(
                    x, y,
                    self.kernel_filter_pixel(x, y, kernel)
                ).unwrap();
            }
        }

        return filtered;
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
    #[ignore]
    fn gaussian_blur() {
        let a = Image::from_file("image/earth.png").unwrap();
        let kernel = filter::Kernel::gaussian(10.0);
        let b = a.kernel_filter(&kernel);
        b.write_to_file("image/blur.out.png").unwrap();
    }
}
