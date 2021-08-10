use std::{path::Path, fs::File, io::Read, fmt::Formatter};
use png::{Decoder, Encoder, ColorType, BitDepth};

/// Color is represented using RGB8.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(u8, u8, u8);

/// Rectangular images represented using RGB8.
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl std::str::FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut transformed: &str = &s.trim().to_lowercase();
        if transformed.starts_with("0x") { transformed = transformed.strip_prefix("0x").unwrap() }
        if transformed.starts_with("#") { transformed = transformed.strip_prefix("#").unwrap() }
        if transformed.len() != 6 { return Err(String::from("Invalid length for color hex code")); }
        let as_number = usize::from_str_radix(transformed, 16).map_err(|e| e.to_string())?;
        let r = (((0b1111_1111 << 16) & as_number) >> 16) as u8; 
        let g = (((0b1111_1111 <<  8) & as_number) >>  8) as u8; 
        let b = (((0b1111_1111 <<  0) & as_number) >>  0) as u8;
        return Ok(Color(r, g, b));
    }
}

impl From<[u8; 3]> for Color {
    fn from(v: [u8; 3]) -> Self {
        Color(v[0], v[1], v[2])
    }
}

impl std::fmt::LowerHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::LowerHex::fmt(&self.0, f)?;
        std::fmt::LowerHex::fmt(&self.0, f)?;
        std::fmt::LowerHex::fmt(&self.0, f)?;
        Ok(())
    }
}

impl std::fmt::UpperHex for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::UpperHex::fmt(&self.0, f)?;
        std::fmt::UpperHex::fmt(&self.1, f)?;
        std::fmt::UpperHex::fmt(&self.2, f)?;
        Ok(())
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        std::fmt::UpperHex::fmt(&self, f)?;
        Ok(())
    }
}

impl Image {
    /// Creates a new all black image
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height,
            data: vec![Color(0, 0, 0); width * height],
        }
    }

    /// Loads an image from a file.
    /// Currently only supports png file.
    pub fn from_file(file: impl AsRef<Path>) -> Result<Self, String> {
        let decoder = {
            let file = File::open(file).map_err(|e| e.to_string())?;
            Decoder::new(file)
        };

        let (info, mut reader) = decoder.read_info().map_err(|e| e.to_string())?;

        let bit_depth = match info.bit_depth {
            BitDepth::One => 1,
            BitDepth::Two => 2,
            BitDepth::Four => 4,
            BitDepth::Eight => 8,
            BitDepth::Sixteen => 16,
        };

        let mut data = {
            let mut data = vec![0u8; reader.output_buffer_size()];
            reader.next_frame(&mut data[..]).map_err(|e| e.to_string())?;
            std::io::Cursor::new(data)
        };

        let mut pixels = Vec::with_capacity(info.width as usize * info.height as usize);

        loop {
            match (info.color_type, bit_depth) {
                (ColorType::Indexed, 1) => {
                    unimplemented!()
                },

                (ColorType::Indexed, 2) => {
                    unimplemented!()
                },

                (ColorType::Indexed, 4) => {
                    unimplemented!()
                },

                (ColorType::Indexed, 8) => {
                    unimplemented!()
                },


                (ColorType::Grayscale, 1) => {
                    let mut byte = [0u8];
                    data.read(&mut byte[..]).unwrap();

                    for i in 7..=0 {
                        if (byte[0] & (0b1 << i)) > 0 { pixels.push(Color(255, 255, 255)); }
                        else { pixels.push(Color(0, 0, 0)); }

                        if pixels.len() == pixels.capacity() { break; }
                    }
                },

                (ColorType::Grayscale, 2) => {
                    let mut byte = [0u8];
                    data.read(&mut byte[..]).unwrap();

                    for i in 3..=0 {
                        let val = (byte[0] & (0b11 << 2*i)) >> 2*i;
                        let val = val * (u8::MAX / 4);

                        pixels.push(Color(val, val, val));

                        if pixels.len() == pixels.capacity() { break; }
                    }
                },

                (ColorType::Grayscale, 4) => {
                    let mut byte = [0u8];
                    data.read(&mut byte[..]).unwrap();

                    for i in 1..=0 {
                        let val = (byte[0] & (0b1111 << 4*i)) >> 4*i;
                        let val = val * (u8::MAX / 16);
                        pixels.push(Color(val, val, val));


                        if pixels.len() == pixels.capacity() { break; }
                    }
                },

                (ColorType::Grayscale, 8) => {
                    let mut byte = [0u8];
                    data.read(&mut byte[..]).unwrap();
                    let val = byte[0];
                    pixels.push(Color(val, val, val));
                },

                (ColorType::Grayscale, 16) => {
                    let mut byte = [0u8; 2];
                    data.read(&mut byte[..]).unwrap();
                    let val = byte[0];
                    pixels.push(Color(val, val, val));
                },


                (ColorType::GrayscaleAlpha, 8) => {
                    let mut byte = [0u8; 2];
                    data.read(&mut byte[..]).unwrap();
                    let val = byte[0];
                    pixels.push(Color(val, val, val));
                },

                (ColorType::GrayscaleAlpha, 16) => {
                    let mut byte = [0u8; 4];
                    data.read(&mut byte[..]).unwrap();
                    let val = byte[0];
                    pixels.push(Color(val, val, val));
                },


                (ColorType::RGB, 8) => {
                    let mut byte = [0u8; 3];
                    data.read(&mut byte[..]).unwrap();
                    let r = byte[0];
                    let g = byte[1];
                    let b = byte[2];
                    pixels.push(Color(r, g, b));
                },

                (ColorType::RGB, 16) => {
                    let mut byte = [0u8; 6];
                    data.read(&mut byte[..]).unwrap();
                    let r = byte[0];
                    let g = byte[2];
                    let b = byte[4];
                    pixels.push(Color(r, g, b));
                },


                (ColorType::RGBA, 8) => {
                    let mut byte = [0u8; 4];
                    data.read(&mut byte[..]).unwrap();
                    let r = byte[0];
                    let g = byte[1];
                    let b = byte[2];
                    pixels.push(Color(r, g, b));
                },

                (ColorType::RGBA, 16) => {
                    let mut byte = [0u8; 8];
                    data.read(&mut byte[..]).unwrap();
                    let r = byte[0];
                    let g = byte[2];
                    let b = byte[4];
                    pixels.push(Color(r, g, b));
                },

                _ => unreachable!(),
            }

            if pixels.len() == pixels.capacity() { break; }
        }

        Ok(Image{width: info.width as usize, height: info.height as usize, data: pixels})
    }
    
    /// Getter for image width
    pub fn width(&self) -> usize { self.width }
    /// Getter for image height
    pub fn height(&self) -> usize { self.height }

    /// Returns the pixel at a given coordinate. Returns None if out of bounds.
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            Some(self.data[x + y * self.width])
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
        let mut encoder = {
            let file = File::create(file).map_err(|e| e.to_string())?;
            Encoder::new(file, self.width as u32, self.height as u32)
        };

        encoder.set_color(ColorType::RGB);
        encoder.set_depth(BitDepth::Eight);

        let mut header = encoder.write_header().map_err(|e| e.to_string())?;

        let mut data = Vec::with_capacity(self.data.len() * 3);
        for c in &self.data {
            data.push(c.0);
            data.push(c.1);
            data.push(c.2);
        }
        
        header.write_image_data(&data[..]).map_err(|e| e.to_string())?;

        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_parsing() {
        use std::str::FromStr;
        let a = Color::from_str("0xFFFFFF").unwrap();
        assert_eq!(Color(255, 255, 255), a);
        let b = Color::from_str("#79a9d9").unwrap();
        assert_eq!(Color(121, 169, 217), b);
        let c = Color::from_str("548941").unwrap();
        assert_eq!(Color(84, 137, 65), c);
    }

    #[test]
    fn read_image_from_file() {
        let _img = Image::from_file("image/test.png").unwrap();
    }

    #[test]
    fn write_image_to_file() {
        let a = Image::from_file("image/test.png").unwrap();
        a.write_to_file("image/test.png.out").unwrap();
        let b = Image::from_file("image/test.png.out").unwrap();

        assert_eq!(a.width(), b.width());
        assert_eq!(a.height(), b.height());
    }
}
