use std::fmt::{Display, Formatter};
use std::hint::unreachable_unchecked;

/// Variable size vector structure.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<const S: usize> {
    data: [f64; S],
}

impl<const S: usize> Vector<S> {
    /// New vector with all 0's.
    pub fn new() -> Self {
        Vector { data: [0.0; S] }
    }

    /// Returns the length of a vector.
    pub fn magnitude(&self) -> f64 {
        (0..S).map(|i| self[i].powf(2.0)).sum::<f64>().sqrt()
    }

    /// Returns the vector with the same direction as self, and a magnitude of 1.
    pub fn normalise(&self) -> Self {
        let mut normalised = self.clone();
        let length = self.magnitude();
        for v in &mut normalised.data {
            *v = *v / length;
        }
        normalised
    }

    /// Static method that calculates the dot product of 2 vectors
    pub fn dot(a: Self, b: Self) -> f64 {
        let mut sum = 0.0;
        for i in 0..S {
            sum += a[i] * b[i];
        }
        return sum;
    }
}

impl Vector<3> {
    /// Calculate the cross product (Only for S=3)
    pub fn cross(a: Self, b: Self) -> Self {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
        .into()
    }
}

/// Addition implementation
impl<const S: usize> std::ops::Add for Vector<S> {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        let mut result = Vector::new();
        for i in 0..S {
            result[i] = self[i] + b[i];
        }
        result
    }
}

impl<const S: usize> std::ops::AddAssign for Vector<S> {
    fn add_assign(&mut self, b: Self) {
        for i in 0..S {
            self[i] += b[i]
        }
    }
}

/// Subtraction implementation
impl<const S: usize> std::ops::Sub for Vector<S> {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        let mut result = Vector::new();
        for i in 0..S {
            result[i] = self[i] - b[i];
        }
        result
    }
}

impl<const S: usize> std::ops::SubAssign for Vector<S> {
    fn sub_assign(&mut self, b: Self) {
        for i in 0..S {
            self[i] -= b[i]
        }
    }
}

/// Scalar multiplication implementation
impl<const S: usize> std::ops::Mul<f64> for Vector<S> {
    type Output = Self;
    fn mul(self, b: f64) -> Self::Output {
        let mut result = self;
        for i in 0..S {
            result[i] *= b;
        }
        result
    }
}

/// Commutative scalar multiplication
impl<const S: usize> std::ops::Mul<Vector<S>> for f64 {
    type Output = Vector<S>;
    fn mul(self, b: Vector<S>) -> Self::Output {
        let mut result = b;
        for i in 0..S {
            result[i] *= self;
        }
        result
    }
}

/// Const indexing
impl<const S: usize> std::ops::Index<usize> for Vector<S> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// Mutable indexing
impl<const S: usize> std::ops::IndexMut<usize> for Vector<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// Conversion from float array
impl<const S: usize> From<[f64; S]> for Vector<S> {
    fn from(data: [f64; S]) -> Self {
        Vector { data }
    }
}

impl <const S: usize> Display for Vector<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for v in self.data.iter().enumerate() {
            if v.0 == S - 1 {
                return write!(f, "{}]", v.1);
            } else {
                write!(f, "{}, ", v.0)?;
            }
        }

        unreachable!();
    }
}