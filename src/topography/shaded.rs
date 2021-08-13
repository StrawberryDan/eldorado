use super::*;

pub struct Settings {
    light_dir: Vector<2>,
    background_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            light_dir: Vector::from([1.0, 1.0]).normalise(),
            background_color: Color::from([0.0, 0.0, 0.0, 0.0]),
        }
    }
}

pub fn generate(heightmap: &HeightMap, settings: Settings) -> Image {
    let mut result = Image::new(heightmap.width(), heightmap.height()).fill(settings.background_color);

    for x in 0..heightmap.width() {
        for y in 0..heightmap.height() {
            let norm = heightmap.normal_at(x, y);

            let mut shading = match norm {
                Some(norm) => Vector::dot(norm, settings.light_dir),
                None => continue,
            };

            let color = if shading < 0.0 {
                Color::mix(settings.background_color, Color::from([255u8, 255, 255]), -shading)
            } else {
                Color::mix(settings.background_color, Color::from([0u8, 0, 0]), shading)
            };

            result.set_pixel_at(x, y, color).unwrap();
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

     #[test]
    pub fn shaded() {
        let image = Image::from_file("image/earth.png").unwrap();
        let heightmap = HeightMap::from(image);
        let shaded = generate(&heightmap, Settings::default());
        shaded.write_to_file("image/heightmap_shaded.out.png").unwrap();
    }
}
