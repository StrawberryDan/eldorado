use crate::image::Image;
use rand::rngs::SmallRng;
use rand::{SeedableRng, RngCore};
use crate::color::Color;
use std::sync::{Arc, Mutex};

const OVERLAP_PREVENTION: usize = 1;

pub struct GlyphDistribution {
    glyph: Image,
    layer_size: (usize, usize),
    locations: Vec<(usize, usize)>,
}

impl GlyphDistribution {
    pub fn new(seed: [u8; 32], density: usize, threshold: usize, glyph: &Image, map: &Image, color: Color) -> Self {
        let mut possible_locations = Vec::new();

        let mut x = 0;
        while x < map.width() {
            let mut y = 0;
            while y < map.height() {
                possible_locations.push((x, y));
                y += glyph.height() + glyph.height() / 2 + OVERLAP_PREVENTION;
            }
            x += glyph.width() + glyph.width() / 2 + OVERLAP_PREVENTION;
        }

        let rng = Arc::new(Mutex::new(SmallRng::from_seed(seed)));

        possible_locations = possible_locations.into_iter()
            // Apply random selection
            .filter(|_| {
                let r_value = rng.lock().unwrap().next_u32() as usize;
                (r_value % 100) < density
            })
            // Remove locations not in region
            .filter(|(x, y)| {
                map.pixel_at(*x as usize, *y as usize).unwrap() == color
            })
            // Wiggle the locations a small amount
            .map(|(x, y)| {
                let r1 = (rng.lock().unwrap().next_u32() as usize % (glyph.width() - OVERLAP_PREVENTION)) as isize - glyph.width() as isize / 2;
                let r2 = (rng.lock().unwrap().next_u32() as usize % (glyph.height() - OVERLAP_PREVENTION)) as isize - glyph.height() as isize / 2;
                (x as isize + r1, y as isize + r2)
            })
            // Remove out of bounds locations and revert to usize
            .filter(|(x, y)| *x > 0 && *y > 0)
            .map(|(x, y)| (x as usize, y as usize))
            // Remove glyphs which overlap with wrong biomes
            .filter(|(x, y)| {
                let gx = *x as isize;
                let gy = *y as isize;

                let mut total = 0;
                let mut in_count = 0;
                for x in 0..glyph.width() {
                    for y in 0..glyph.height() {
                        let map_pixel = map.pixel_at(
                            (gx + x as isize - glyph.width() as isize / 2) as usize,
                            (gy + y as isize - glyph.height() as isize / 2) as usize,
                        ).unwrap();

                        if glyph.pixel_at(x, y).unwrap() != Color::from([0, 0, 0, 0]) {
                            if map_pixel == color {
                                in_count += 1;
                            }

                            total += 1;
                        }
                    }
                }

                let ratio = in_count as f64 / total as f64;
                return ratio >= (threshold as f64 / 100.0);
            })
            .collect();

        return GlyphDistribution {
            glyph: glyph.clone(),
            layer_size: (map.width(), map.height()),
            locations: possible_locations,
        };
    }

    pub fn to_layer(&self) -> Image {
        let mut layer = Image::new(self.layer_size.0, self.layer_size.1);
        for location in &self.locations {
            let x = match location.0.checked_sub(self.glyph.width() / 2) {
                Some(v) => v,
                None => continue,
            };

            let y = match location.1.checked_sub(self.glyph.height() / 2) {
                Some(v) => v,
                None => continue,
            };

            layer.stamp((x, y), &self.glyph);
        }
        return layer;
    }
}
