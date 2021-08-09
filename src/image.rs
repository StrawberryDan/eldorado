use std::{path::Path, fs::File, io::Read};
use png::{Decoder, Encoder, ColorType, BitDepth};

#[derive(Copy, Clone, Debug)]
pub struct Color(u8, u8, u8);

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height,
            data: vec![Color(0, 0, 0); width * height],
        }
    }

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

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

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
