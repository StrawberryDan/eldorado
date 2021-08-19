use super::*;

pub struct Settings {
    /// Number of contour lines between 0 and u16::MAX.
    line_divisions: u16,
    /// Color of points facing the light dir.
    light_color: Color,
    /// Color of points facing away from the light dir.
    dark_color: Color,
    /// Color to point cells with no contour line on them.
    background_color: Color,
    /// Direction that light will shine from in image space.
    light_dir: Vector<2>,

    cleaning_factor: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            line_divisions: 32,
            light_color: Color::from([1.0, 1.0, 1.0]),
            dark_color: Color::from([0.0, 0.0, 0.0]),
            background_color: Color::from([0.0, 0.0, 0.0, 0.0]),
            light_dir: Vector::from([1.0, 1.0]),
            cleaning_factor: 2,
        }
    }
}

pub fn generate(heightmap: &HeightMap, settings: Settings) -> Image {
    let mut tanaka = Image::new(heightmap.width(), heightmap.height());

    let contour_line_color = Color::from([1.0, 0.0, 0.0]);
    let contours = super::generate_contour_layer(
        heightmap,
        super::ContourSettings {
            line_divisions: settings.line_divisions,
            line_color: contour_line_color,
            cleaning_factor: settings.cleaning_factor,
            ..Default::default()
        },
    );

    for x in 0..heightmap.width() {
        for y in 0..heightmap.height() {
            if contours.pixel_at(x, y).unwrap() == contour_line_color {
                // Work out light value
                let normal = heightmap.normal_at(x, y);
                let light = match normal {
                    Some(normal) => Vector::dot(settings.light_dir.normalise(), normal),
                    None => continue,
                };

                // Paint cell
                let c = if light < 0.0 {
                    Color::interpolate(settings.background_color, settings.light_color, -light)
                } else {
                    Color::interpolate(settings.background_color, settings.dark_color, light)
                };

                tanaka.set_pixel_at(x, y, c).unwrap();
            } else {
                tanaka
                    .set_pixel_at(x, y, settings.background_color)
                    .unwrap();
            }
        }
    }

    return tanaka;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tanaka() {
        let image = Image::from_file("image/earth.png").unwrap();
        let heightmap = HeightMap::from(image);

        let contours = generate(&heightmap, Settings::default());

        contours
            .write_to_file("image/tanaka_contours.out.png")
            .unwrap();
    }
}
