use super::*;
use png::{BitDepth, ColorType, Encoder};

#[cfg(target_endian = "big")]
fn to_network_endianness(bytes: Vec<u8>) -> Vec<u8> {
    bytes
}

#[cfg(target_endian = "little")]
fn to_network_endianness(mut bytes: Vec<u8>) -> Vec<u8> {
    bytes.reverse();
    return bytes;
}

pub fn write_file(image: &Image, file: impl AsRef<Path>) -> Result<(), String> {
    let mut encoder = {
        let file = File::create(file).map_err(|e| e.to_string())?;
        Encoder::new(file, image.width as u32, image.height as u32)
    };

    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Sixteen);

    let mut header = encoder.write_header().map_err(|e| e.to_string())?;

    let mut data = Vec::with_capacity(image.data.len() * 4);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let c = image.pixel_at(x, y).unwrap();
            let values = c.as_u16();
            for v in values {
                let bytes = unsafe { std::mem::transmute::<u16, [u8;2]>(v) };
                let bytes = Vec::from(&bytes[..]);
                let bytes = to_network_endianness(bytes);
                for b in bytes {
                    data.push(b);
                }
            }
        }
    }

    let as_bytes = unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * std::mem::size_of::<u8>(),
        )
    };

    header
        .write_image_data(&as_bytes[..])
        .map_err(|e| e.to_string())?;

    return Ok(());
}
