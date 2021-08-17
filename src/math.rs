pub fn gauss(x: f64, mean: f64, sd: f64) -> f64 {
    let multiplicand = 1.0 / ( sd * f64::sqrt(2.0 * std::f64::consts::PI) );
    let exponent = (-1.0 / 2.0) * ( (x - mean) / sd).powf(2.0);
    return multiplicand * std::f64::consts::E.powf(exponent);
}
