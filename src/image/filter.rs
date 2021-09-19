use std::collections::HashMap;

const KERNEL_MINIMUM_VALUE: f64 = 0.0000001;

#[derive(Clone)]
pub struct Kernel {
    values: HashMap<(isize, isize), f64>,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel { values: HashMap::new() }
    }

    pub fn gaussian(sd: f64) -> Self {
        let mut kernel = Kernel::new();
        let radius = (2.0 * sd * sd).ceil() as isize;

        for x in -radius..radius {
            for y in -radius..radius {
                use std::f64::consts::{PI, E};

                let xc = x as f64;
                let yc = y as f64;

                let v = ( 1.0 / (2.0 * PI * sd * sd) ) * E.powf( -(xc * xc  + yc * yc) / ( 2.0 * sd * sd) );
                if v < KERNEL_MINIMUM_VALUE { continue; }
                *kernel.value_at_mut((xc as isize, yc as isize)) = v;
            }
        }

        return kernel;
    }

    pub fn value_at(&self, p: (isize, isize)) -> f64 {
        return *self.values.get(&p).unwrap_or(&0.0);
    }

    pub fn value_at_mut(&mut self, p: (isize, isize)) -> &mut f64 {
        if !self.values.contains_key(&p) {
            self.values.insert(p, 0.0);
        } 

        return self.values.get_mut(&p).unwrap();
    }

    pub fn pairs(&self) -> Vec<((isize, isize), f64)> {
        return self.values.iter().map(|(p, v)| (*p, *v)).collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gauss_test() {
        let k = Kernel::gaussian(0.84089642);
        
        assert!(k.value_at((0, 0)) - 0.22508352 < 0.00000001); 
        assert!(k.value_at((-3, 0)) - 0.00038771 < 0.00000001);
        assert!(k.value_at((2, -1)) - 0.00655965 < 0.00000001);
    }
}
