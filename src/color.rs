use crate::vector::Vector;
use std::fmt::Formatter;
use std::convert::TryFrom;

/// Color is represented using RGB8.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color([f64; 4]);

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
    pub fn interpolate(a: Color, b: Color, factor: f64) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        let R = a[0] * (1.0 - factor) + b[0] * factor;
        let G = a[1] * (1.0 - factor) + b[1] * factor;
        let B = a[2] * (1.0 - factor) + b[2] * factor;
        let A = a[3] * (1.0 - factor) + b[3] * factor;
        Color([R, G, B, A])
    }

    pub fn as_vector(&self) -> Vector<4> {
        Vector::from(self.0)
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
        if transformed.len() == 6 {
            let as_number = u32::from_str_radix(transformed, 16).map_err(|e| e.to_string())?;
            let r = (((0b1111_1111 << 16) & as_number) >> 16) as u8;
            let g = (((0b1111_1111 << 8) & as_number) >> 8) as u8;
            let b = (((0b1111_1111 << 0) & as_number) >> 0) as u8;

            let r = r as f64 / u8::MAX as f64;
            let g = g as f64 / u8::MAX as f64;
            let b = b as f64 / u8::MAX as f64;

            return Ok(Color([r, g, b, 1.0]));
        } else if transformed.len() == 8 {
            let as_number = u32::from_str_radix(transformed, 16).map_err(|e| e.to_string())?;
            let r = (((0b1111_1111 << 24) & as_number) >> 24) as u8;
            let g = (((0b1111_1111 << 16) & as_number) >> 16) as u8;
            let b = (((0b1111_1111 << 8) & as_number) >> 8) as u8;
            let a = (((0b1111_1111 << 0) & as_number) >> 0) as u8;

            let r = r as f64 / u8::MAX as f64;
            let g = g as f64 / u8::MAX as f64;
            let b = b as f64 / u8::MAX as f64;
            let a = a as f64 / u8::MAX as f64;

            return Ok(Color([r, g, b, a]));
        } else {
            return Err(String::from("Invalid hex string length for color"));
        }
    }
}

impl From<[u8; 3]> for Color {
    fn from(v: [u8; 3]) -> Self {
        let v = v
            .into_iter()
            .map(|v| *v as f64 / u8::MAX as f64)
            .collect::<Vec<_>>();
        Color([v[0], v[1], v[2], 1.0])
    }
}

impl From<[u8; 4]> for Color {
    fn from(v: [u8; 4]) -> Self {
        let v = v
            .into_iter()
            .map(|v| *v as f64 / u8::MAX as f64)
            .collect::<Vec<_>>();
        Color([v[0], v[1], v[2], v[3]])
    }
}

impl From<[u16; 3]> for Color {
    fn from(v: [u16; 3]) -> Self {
        let v = v
            .into_iter()
            .map(|v| *v as f64 / u16::MAX as f64)
            .collect::<Vec<_>>();
        Color([v[0], v[1], v[2], 1.0])
    }
}

impl From<[u16; 4]> for Color {
    fn from(v: [u16; 4]) -> Self {
        let v = v
            .into_iter()
            .map(|v| *v as f64 / u16::MAX as f64)
            .collect::<Vec<_>>();
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

impl TryFrom<Vector<3>> for Color {
    type Error = String;
    fn try_from(v: Vector<3>) -> Result<Self, Self::Error> {
        for i in 0..2 {
            if v[i] < 0.0 || v[i] > 1.0 { return Err(String::from("Color value out of bounds")); }
        }

        Ok(Color{ 0: [v[0], v[1], v[2], 1.0] })
    }
}

impl TryFrom<Vector<4>> for Color {
    type Error = String;
    fn try_from(v: Vector<4>) -> Result<Self, Self::Error> {
        for i in 0..3 {
            if v[i] < 0.0 || v[i] > 1.0 { return Err(String::from("Color value out of bounds")); }
        }

        Ok(Color{ 0: [v[0], v[1], v[2], v[3]] })
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{}, {}, {}, {}]", self[0], self[1], self[2], self[3])
    }
}
