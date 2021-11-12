use std::fmt::Formatter;

/// Color is represented using normalised floating points.
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Color([u8; 4]);

impl std::ops::Index<usize> for Color {
    type Output = u8;
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
    pub fn interpolate(x: Color, y: Color, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        let r = x[0] as f64  * (1.0 - factor) + y[0] as f64 * factor;
        let g = x[1] as f64  * (1.0 - factor) + y[1] as f64  * factor;
        let b = x[2]  as f64 * (1.0 - factor) + y[2] as f64  * factor;
        let a = x[3] as f64  * (1.0 - factor) + y[3] as f64  * factor;
        Color([r.round() as u8, g.round() as u8, b.round() as u8, a.round() as u8])
    }
}

impl From<[u8; 3]> for Color {
    fn from(v: [u8; 3]) -> Self {
        Color([v[0], v[1], v[2], 255])
    }
}

impl std::str::FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut transformed: &str = &s.trim().to_lowercase();
        if transformed.starts_with("0x") {
            transformed = transformed.strip_prefix("0x").unwrap()
        }
        if transformed.starts_with("#") {
            transformed = transformed.strip_prefix("#").unwrap()
        }
        return if transformed.len() == 6 {
            let as_number = u32::from_str_radix(transformed, 16).map_err(|e| e.to_string())?;
            let r = (((0b1111_1111 << 16) & as_number) >> 16) as u8;
            let g = (((0b1111_1111 << 8) & as_number) >> 8) as u8;
            let b = (((0b1111_1111 << 0) & as_number) >> 0) as u8;

            Ok(Color([r, g, b, 255]))
        } else if transformed.len() == 8 {
            let as_number = u32::from_str_radix(transformed, 16).map_err(|e| e.to_string())?;
            let r = (((0b1111_1111 << 24) & as_number) >> 24) as u8;
            let g = (((0b1111_1111 << 16) & as_number) >> 16) as u8;
            let b = (((0b1111_1111 << 8) & as_number) >> 8) as u8;
            let a = (((0b1111_1111 << 0) & as_number) >> 0) as u8;

            Ok(Color([r, g, b, a]))
        } else {
            Err(String::from("Invalid hex string length for color"))
        }
    }

}

impl From<[u8; 4]> for Color {
    fn from(v: [u8; 4]) -> Self {
        Color([v[0], v[1], v[2], v[3]])
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{}, {}, {}, {}]", self[0], self[1], self[2], self[3])
    }
}
