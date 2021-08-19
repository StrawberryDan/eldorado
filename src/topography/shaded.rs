use super::*;

pub struct Settings {
    /// Direction in image space from where light will shine
    light_dir: Vector<2>,
    /// Color to use for cells with no shading.
    background_color: Color,
    /// Color to use on cells facing the light.
    light_color: Color,
    /// Color to use for cells facing away from the light.
    dark_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            light_dir: Vector::from([1.0, 1.0]).normalise(),
            background_color: Color::from([0.0, 0.0, 0.0, 0.0]),
            light_color: Color::from([1.0, 1.0, 1.0, 1.0]),
            dark_color: Color::from([0.0, 0.0, 0.0, 1.0]),
        }
    }
}

pub fn generate(heightmap: &HeightMap, settings: Settings) -> Image {
    let mut result =
        Image::new(heightmap.width(), heightmap.height()).fill(settings.background_color);

    for x in 0..heightmap.width() {
        for y in 0..heightmap.height() {
            // Work out lighting value
            let norm = heightmap.normal_at(x, y);
            let shading = match norm {
                Some(norm) => Vector::dot(norm, settings.light_dir),
                None => continue,
            };

            // Interpolate between colors
            let color = if shading < 0.0 {
                Color::interpolate(settings.background_color, settings.light_color, -shading)
            } else {
                Color::interpolate(settings.background_color, settings.dark_color, shading)
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
        shaded
            .write_to_file("image/heightmap_shaded.out.png")
            .unwrap();
    }
}
