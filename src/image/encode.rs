use super::*;
use png::{BitDepth, ColorType, Encoder};

pub fn write_file(image: &Image, file: impl AsRef<Path>) -> Result<(), String> {
    let mut encoder = {
        let file = File::create(file).map_err(|e| e.to_string())?;
        Encoder::new(file, image.width as u32, image.height as u32)
    };

    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Sixteen);

    let mut header = encoder.write_header().map_err(|e| e.to_string())?;

    let mut data = Vec::with_capacity(image.data.len() * 3);
    for c in &image.data {
        let r = c[0] * u16::MAX as f64;
        let g = c[1] * u16::MAX as f64;
        let b = c[2] * u16::MAX as f64;
        let a = c[3] * u16::MAX as f64;

        data.push(r.round() as u16);
        data.push(g.round() as u16);
        data.push(b.round() as u16);
        data.push(a.round() as u16);
    }

    let as_bytes = unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * std::mem::size_of::<u16>(),
        )
    };

    header
        .write_image_data(&as_bytes[..])
        .map_err(|e| e.to_string())?;

    return Ok(());
}
