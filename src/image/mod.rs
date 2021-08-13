use std::{path::Path, fs::File, io::Read, fmt::Formatter};
use png::{Decoder, Encoder, ColorType, BitDepth};

/// Color is represented using RGB8.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color([f64; 4]);

/// Rectangular images represented using RGB8.
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl std::ops::Index<usize> for Color {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
} 

impl std::ops::IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
} 

impl Color {
    pub fn mix(a: Color, b: Color, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        let R = a[0] * (1.0 - factor) + b[0] * factor;
        let G = a[1] * (1.0 - factor) + b[1] * factor;
        let B = a[2] * (1.0 - factor) + b[2] * factor;
        let A = a[3] * (1.0 - factor) + b[3] * factor;
        Color([R, G, B, A])
    }
}

impl std::str::FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut transformed: &str = &s.trim().to_lowercase();
        if transformed.starts_with("0x") { transformed = transformed.strip_prefix("0x").unwrap() }
        if transformed.starts_with("#") { transformed = transformed.strip_prefix("#").unwrap() }
        if transformed.len() == 6 {
            let as_number = u32::from_str_radix(transformed, 16).map_err(|e| e.to_string())?;
            let r = (((0b1111_1111 << 16) & as_number) >> 16) as u8; 
            let g = (((0b1111_1111 <<  8) & as_number) >>  8) as u8; 
            let b = (((0b1111_1111 <<  0) & as_number) >>  0) as u8;

            let r = r as f64 / u8::MAX as f64;
            let g = g as f64 / u8::MAX as f64;
            let b = b as f64 / u8::MAX as f64;
            
            return Ok(Color([r, g, b, 1.0]));
        } else if transformed.len() == 8 {
            let as_number = u32::from_str_radix(transformed, 16).map_err(|e| e.to_string())?;
            let r = (((0b1111_1111 << 24) & as_number) >> 24) as u8;
            let g = (((0b1111_1111 << 16) & as_number) >> 16) as u8;
            let b = (((0b1111_1111 <<  8) & as_number) >>  8) as u8;
            let a = (((0b1111_1111 <<  0) & as_number) >>  0) as u8;

            let r = r as f64 / u8::MAX as f64;
            let g = g as f64 / u8::MAX as f64;
            let b = b as f64 / u8::MAX as f64;
            let a = a as f64 / u8::MAX as f64;
            
            return Ok(Color([r, g, b, a]));
        } else {
            return Err( String::from("Invalid hex string length for color") );
        }
    }
}

impl From<[u8; 3]> for Color {
    fn from(v: [u8; 3]) -> Self {
        let v = v.into_iter().map(|v| *v as f64 / u8::MAX as f64).collect::<Vec<_>>();
        Color([v[0], v[1], v[2], 1.0])
    }
}

impl From<[u8; 4]> for Color {
    fn from(v: [u8; 4]) -> Self {
        let v = v.into_iter().map(|v| *v as f64 / u8::MAX as f64).collect::<Vec<_>>();
        Color([v[0], v[1], v[2], v[3]])
    }
}

impl From<[u16; 3]> for Color {
    fn from(v: [u16; 3]) -> Self {
        let v = v.into_iter().map(|v| *v as f64 / u16::MAX as f64).collect::<Vec<_>>();
        Color([v[0], v[1], v[2], 1.0])
    }
}

impl From<[u16; 4]> for Color {
    fn from(v: [u16; 4]) -> Self {
        let v = v.into_iter().map(|v| *v as f64 / u16::MAX as f64).collect::<Vec<_>>();
        Color([v[0], v[1], v[2], v[3]])
    }
}

impl From<[f64; 3]> for Color {
    fn from(v: [f64; 3]) -> Self {
        Color([v[0], v[1], v[2], 1.0])
    }
}

impl From<[f64; 4]> for Color {
    fn from(v: [f64; 4]) -> Self {
        Color(v)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{}, {}, {}, {}]", self[0], self[1], self[2], self[3])
    }
}

impl Image {
    /// Creates a new all black image
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height,
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
                        if (byte[0] & (0b1 << i)) > 0 { pixels.push(Color::from([255u8, 255, 255])); }
                        else { pixels.push(Color::from([0u8, 0, 0])); }

                        if pixels.len() == pixels.capacity() { break; }
                    }
                },

                (ColorType::Grayscale, 2) => {
                    let mut byte = [0u8];
                    data.read(&mut byte[..]).unwrap();

                    for i in 3..=0 {
                        let val = (byte[0] & (0b11 << 2*i)) >> 2*i;
                        let val = val * (u8::MAX / 4);

                        pixels.push(Color::from([val, val, val]));

                        if pixels.len() == pixels.capacity() { break; }
                    }
                },

                (ColorType::Grayscale, 4) => {
                    let mut byte = [0u8];
                    data.read(&mut byte[..]).unwrap();

                    for i in 1..=0 {
                        let val = (byte[0] & (0b1111 << 4*i)) >> 4*i;
                        let val = val * (u8::MAX / 16);
                        pixels.push(Color::from([val, val, val]));


                        if pixels.len() == pixels.capacity() { break; }
                    }
                },

                (ColorType::Grayscale, 8) => {
                    let mut byte = [0u8; 1];
                    data.read(&mut byte[..]).unwrap();
                    let val = byte[0];
                    pixels.push(Color::from([val, val, val]));
                },

                (ColorType::Grayscale, 16) => {
                    let mut byte = [0u8; 2];
                    data.read(&mut byte[..]).unwrap();
                    let val = u16::from_le_bytes(byte);
                    pixels.push(Color::from([val, val, val]));
                },


                (ColorType::GrayscaleAlpha, 8) => {
                    let mut byte = [0u8; 2];
                    data.read(&mut byte[..]).unwrap();
                    let g = byte[0];
                    let a = byte[1];
                    pixels.push(Color::from([g, g, g, a]));
                },

                (ColorType::GrayscaleAlpha, 16) => {
                    let mut byte = [0u8; 4];
                    data.read(&mut byte[..]).unwrap();
                    let mut byte = unsafe { std::mem::transmute::<_, [u16; 2]>(byte) };
                    let g = byte[0];
                    let a = byte[1];
                    pixels.push(Color::from([g, g, g, a]));
                },


                (ColorType::RGB, 8) => {
                    let mut byte = [0u8; 3];
                    data.read(&mut byte[..]).unwrap();
                    let r = byte[0];
                    let g = byte[1];
                    let b = byte[2];
                    pixels.push(Color::from([r, g, b]));
                },

                (ColorType::RGB, 16) => {
                    let mut byte = [0u8; 6];
                    data.read(&mut byte[..]).unwrap();
                    let mut byte = unsafe { std::mem::transmute::<_, [u16; 3]>(byte) };
                    let r = byte[0];
                    let g = byte[1];
                    let b = byte[2];
                    pixels.push(Color::from([r, g, b]));
                },


                (ColorType::RGBA, 8) => {
                    let mut byte = [0u8; 4];
                    data.read(&mut byte[..]).unwrap();
                    let r = byte[0];
                    let g = byte[1];
                    let b = byte[2];
                    let a = byte[3];
                    pixels.push(Color::from([r, g, b, a]));
                },

                (ColorType::RGBA, 16) => {
                    let mut byte = [0u8; 8];
                    data.read(&mut byte[..]).unwrap();
                    let mut byte = unsafe { std::mem::transmute::<_, [u16; 4]>(byte) };
                    let r = byte[0];
                    let g = byte[1];
                    let b = byte[2];
                    let a = byte[3];
                    pixels.push(Color::from([r, g, b, a]));
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
    /// Getter for the images raw pixel data
    pub fn data(&self) -> &Vec<Color> { &self.data }

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

        encoder.set_color(ColorType::RGBA);
        encoder.set_depth(BitDepth::Sixteen);

        let mut header = encoder.write_header().map_err(|e| e.to_string())?;

        let mut data = Vec::with_capacity(self.data.len() * 3);
        for c in &self.data {
            let r = c[0] * u16::MAX as f64;
            let g = c[1] * u16::MAX as f64;
            let b = c[2] * u16::MAX as f64;
            let a = c[3] * u16::MAX as f64;

            data.push(r.round() as u16);
            data.push(g.round() as u16);
            data.push(b.round() as u16);
            data.push(a.round() as u16);
        }

        let as_bytes = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<u16>()) };
        
        header.write_image_data(&as_bytes[..]).map_err(|e| e.to_string())?;

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
