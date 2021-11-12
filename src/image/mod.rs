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

    /// Returns a pixel at a given coordinate but using signed coordinates
    pub fn pixel_at_isize(&self, x: isize, y: isize) -> Option<Color> {
        if x < 0 || y < 0 {
            None
        } else {
            self.pixel_at(x as usize, y as usize)
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

    /// Sets the pixel at a given coordinate using signed coordinates, returns aan error if out of bounds.
    pub fn set_pixel_at_isize(&mut self, x: isize, y: isize, c: Color) -> Result<(), String> {
        if x < 0 || y < 0 {
            Err(String::from("Cannot set pixel at negative coordinates"))
        } else {
            self.set_pixel_at(x as usize, y as usize, c)
        }
    }

    /// Returns the colors and positions of pixels neighbouring (x, y).
    pub fn get_neighbouring_pixels(&self, x: isize, y: isize) -> Vec<((usize, usize), Color)> {
        let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        return offsets.iter()
            .map(|o| (o, self.pixel_at_isize(x + o.0, y + o.1)))
            .filter(|(_, c)| c.is_some())
            .map(|(o, c)| ((o.0 as usize, 0.1 as usize), c.unwrap()))
            .collect();
    }

    /// Writes the image to a file
    /// Currently only supports png files.
    pub fn write_to_file(&self, file: impl AsRef<Path>) -> Result<(), String> {
        encode::write_file(self, file)
    }

    pub fn kernel_filter_pixel(&self, x: usize, y: usize, kernel: &filter::Kernel) -> Color {
        use std::convert::TryInto;

        let mut color_acc = [0u64; 4];
        let mut value_acc = 0.0;

        for (offset, value) in kernel.pairs() {
            let x = x as isize + offset.0;
            let y = y as isize + offset.1;

            let color = self.pixel_at(x as usize, y as usize);

            match color {
                Some(c) => {
                    for i in 0..4 {
                        color_acc[i] += c[i] as u64;
                    }

                    value_acc += value;
                }

                None => continue,
            }
        }

        let color_acc: [u8; 4] = color_acc.map(|v| (v as f64 / value_acc).round() as u8).try_into().unwrap();
        return color_acc.into();
    }

    pub fn kernel_filter(&self, kernel: &filter::Kernel) -> Image {
        let mut filtered = self.clone();

        for x in 0..filtered.width() {
            for y in 0..filtered.height() {
                filtered.set_pixel_at(
                    x, y,
                    self.kernel_filter_pixel(x, y, kernel),
                ).unwrap();
            }
        }

        return filtered;
    }

    pub fn stamp(&mut self, pos: (usize, usize), stamp: &Image) {
        for x in 0..stamp.width() {
            for y in 0..stamp.height() {
                let gx = x + pos.0;
                let gy = y + pos.1;

                if gx >= self.width || gy >= self.height {
                    continue;
                }

                let top_color = stamp.pixel_at(x, y).unwrap();
                let bottom_color = self.pixel_at(gx, gy).unwrap();

                let factor = top_color[3] as f64 / u8::MAX as f64;

                let mixed = Color::interpolate(bottom_color, top_color, factor);

                self.set_pixel_at(gx, gy, mixed).unwrap();
            }
        }
    }

    pub fn overlay(&mut self, top: &Image) -> Result<(), String> {
        if self.width != top.width || self.height != top.height {
            return Err(String::from("Images not the same size, cannot overlay"));
        }

        for x in 0..self.width() {
            for y in 0..self.height() {
                let top_color = top.pixel_at(x, y).unwrap();
                let bottom_color = self.pixel_at(x, y).unwrap();

                let factor = top_color[3] as f64 / u8::MAX as f64;

                let mixed = Color::interpolate(bottom_color, top_color, factor);

                self.set_pixel_at(x, y, mixed).unwrap();
            }
        }

        return Ok(());
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
