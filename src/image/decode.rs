use super::*;
use png::{BitDepth, ColorType, Decoder};

pub fn load_png(file: impl AsRef<Path>) -> Result<Image, String> {
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
        reader
            .next_frame(&mut data[..])
            .map_err(|e| e.to_string())?;
        std::io::Cursor::new(data)
    };

    let mut pixels = Vec::with_capacity(info.width as usize * info.height as usize);

    loop {
        match (info.color_type, bit_depth) {
            (ColorType::Indexed, 1) => {
                unimplemented!()
            }

            (ColorType::Indexed, 2) => {
                unimplemented!()
            }

            (ColorType::Indexed, 4) => {
                unimplemented!()
            }

            (ColorType::Indexed, 8) => {
                unimplemented!()
            }

            (ColorType::Grayscale, 1) => {
                let mut byte = [0u8];
                data.read(&mut byte[..]).unwrap();

                for i in 7..=0 {
                    if (byte[0] & (0b1 << i)) > 0 {
                        pixels.push(Color::from([255u8, 255, 255]));
                    } else {
                        pixels.push(Color::from([0u8, 0, 0]));
                    }

                    if pixels.len() == pixels.capacity() {
                        break;
                    }
                }
            }

            (ColorType::Grayscale, 2) => {
                let mut byte = [0u8];
                data.read(&mut byte[..]).unwrap();

                for i in 3..=0 {
                    let val = (byte[0] & (0b11 << 2 * i)) >> 2 * i;
                    let val = val * (u8::MAX / 4);

                    pixels.push(Color::from([val, val, val]));

                    if pixels.len() == pixels.capacity() {
                        break;
                    }
                }
            }

            (ColorType::Grayscale, 4) => {
                let mut byte = [0u8];
                data.read(&mut byte[..]).unwrap();

                for i in 1..=0 {
                    let val = (byte[0] & (0b1111 << 4 * i)) >> 4 * i;
                    let val = val * (u8::MAX / 16);
                    pixels.push(Color::from([val, val, val]));

                    if pixels.len() == pixels.capacity() {
                        break;
                    }
                }
            }

            (ColorType::Grayscale, 8) => {
                let mut byte = [0u8; 1];
                data.read(&mut byte[..]).unwrap();
                let val = byte[0];
                pixels.push(Color::from([val, val, val]));
            }

            (ColorType::Grayscale, 16) => {
                let mut byte = [0u8; 2];
                data.read(&mut byte[..]).unwrap();
                let val = u16::from_le_bytes(byte);
                pixels.push(Color::from([val, val, val]));
            }

            (ColorType::GrayscaleAlpha, 8) => {
                let mut byte = [0u8; 2];
                data.read(&mut byte[..]).unwrap();
                let g = byte[0];
                let a = byte[1];
                pixels.push(Color::from([g, g, g, a]));
            }

            (ColorType::GrayscaleAlpha, 16) => {
                let mut byte = [0u8; 4];
                data.read(&mut byte[..]).unwrap();
                let byte = unsafe { std::mem::transmute::<_, [u16; 2]>(byte) };
                let g = byte[0];
                let a = byte[1];
                pixels.push(Color::from([g, g, g, a]));
            }

            (ColorType::RGB, 8) => {
                let mut byte = [0u8; 3];
                data.read(&mut byte[..]).unwrap();
                let r = byte[0];
                let g = byte[1];
                let b = byte[2];
                pixels.push(Color::from([r, g, b]));
            }

            (ColorType::RGB, 16) => {
                let mut byte = [0u8; 6];
                data.read(&mut byte[..]).unwrap();
                let byte = unsafe { std::mem::transmute::<_, [u16; 3]>(byte) };
                let r = byte[0];
                let g = byte[1];
                let b = byte[2];
                pixels.push(Color::from([r, g, b]));
            }

            (ColorType::RGBA, 8) => {
                let mut byte = [0u8; 4];
                data.read(&mut byte[..]).unwrap();
                let r = byte[0];
                let g = byte[1];
                let b = byte[2];
                let a = byte[3];
                pixels.push(Color::from([r, g, b, a]));
            }

            (ColorType::RGBA, 16) => {
                let mut byte = [0u8; 8];
                data.read(&mut byte[..]).unwrap();
                let byte = unsafe { std::mem::transmute::<_, [u16; 4]>(byte) };
                let r = byte[0];
                let g = byte[1];
                let b = byte[2];
                let a = byte[3];
                pixels.push(Color::from([r, g, b, a]));
            }

            _ => unreachable!(),
        }

        if pixels.len() == pixels.capacity() {
            break;
        }
    }

    Ok(Image {
        width: info.width as usize,
        height: info.height as usize,
        data: pixels,
    })
}
