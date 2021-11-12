use super::*;
use png::{BitDepth, ColorType, Encoder};

pub fn write_file(image: &Image, file: impl AsRef<Path>) -> Result<(), String> {
    let mut encoder = {
        let file = File::create(file).map_err(|e| e.to_string())?;
        Encoder::new(file, image.width as u32, image.height as u32)
    };

    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);

    let mut header = encoder.write_header().map_err(|e| e.to_string())?;

    let mut data = Vec::with_capacity(image.data.len() * 4);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let c = image.pixel_at(x, y).unwrap();
            for i in 0..4{
                    data.push(c[i]);
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
