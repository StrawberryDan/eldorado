use super::*;

pub struct Settings {
    /// Direction in image space from where light will shine
    light_dir: Vector<3>,
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
            light_dir: Vector::from([1.0, 1.0, 0.0]).normalise(),
            background_color: Color::from([0, 128, 0, 255]),
            light_color: Color::from([255, 255, 255, 255]),
            dark_color: Color::from([0, 0, 0, 255]),
        }
    }
}

pub fn generate(heightmap: &HeightMap, settings: Settings) -> Image {
    let mut result =
        Image::new(heightmap.width(), heightmap.height()).fill(settings.background_color);

    for x in 0..heightmap.width() {
        for y in 0..heightmap.height() {
            if (x == 53 && y == 97) {
                println!("Here");
            }

            // Work out lighting value
            let norm = heightmap.surface_normal(x, y);
            let shading = Vector::dot(norm, settings.light_dir);

            let color = if shading == 0.0 {
                settings.background_color
            } else {
                Color::interpolate(settings.light_color, settings.dark_color, (shading / 2.0) + 0.5)
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
    #[ignore]
    pub fn shaded() {
        let heightmap = HeightMap::from_file("image/earth.png").unwrap();
        let shaded = generate(&heightmap, Settings::default());
        shaded
            .write_to_file("image/heightmap_shaded.out.png")
            .unwrap();
    }
}
