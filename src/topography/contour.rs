use super::*;

#[derive(Clone, Copy)]
pub struct Settings {
    /// The number of contour lines between 0 and u16::MAX.
    pub line_divisions: u16,
    /// The color to paint the contour lines.
    pub line_color: Color,
    /// The color to paint the space without contour lines.
    pub background_color: Color,

    pub cleaning_factor: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            line_color: Color::from([255, 0, 0]),
            background_color: Color::from([0, 0, 0, 0]),
            line_divisions: 32,
            cleaning_factor: 2,
        }
    }
}

pub fn generate(heightmap: &HeightMap, settings: Settings) -> Image {
    let mut contours = Image::new(heightmap.width(), heightmap.height());
    // Create a heightmap with each point rounded down to multiples of division size.
    let division_size = u16::MAX / settings.line_divisions;
    let mut leveled = heightmap.clone();
    leveled
        .set_data(leveled.data().iter().map(|v| v / division_size).collect())
        .unwrap();
    // Work out if each cell is on a ridge in the leveled map and, if so, color it.
    for x in 0..heightmap.width() {
        for y in 0..heightmap.height() {
            let v = leveled.height_at(x, y).unwrap();
            let neighbours = leveled.orthogonal_neighbours(x, y);
            let lower_count = neighbours.iter().filter(|(_, nv)| v > *nv).count();

            contours
                .set_pixel_at(
                    x,
                    y,
                    if lower_count > 0 {
                        settings.line_color
                    } else {
                        settings.background_color
                    },
                )
                .unwrap();
        }
    }

    clean_contours(&mut contours, settings);

    return contours;
}

fn clean_contours(image: &mut Image, settings: Settings) {
    if settings.cleaning_factor == 0 {
        return;
    } else {
        for x in 1..image.width() - 1 {
            for y in 1..image.height() - 1 {
                if image.pixel_at(x, y).unwrap() == settings.line_color {
                    let mut neighbours = 0;
                    if image.pixel_at(x - 1, y - 1).unwrap() == settings.line_color {
                        neighbours += 1;
                    }
                    if image.pixel_at(x - 1, y + 0).unwrap() == settings.line_color {
                        neighbours += 1;
                    }
                    if image.pixel_at(x - 1, y + 1).unwrap() == settings.line_color {
                        neighbours += 1;
                    }
                    if image.pixel_at(x + 0, y - 1).unwrap() == settings.line_color {
                        neighbours += 1;
                    }
                    if image.pixel_at(x + 0, y + 1).unwrap() == settings.line_color {
                        neighbours += 1;
                    }
                    if image.pixel_at(x + 1, y - 1).unwrap() == settings.line_color {
                        neighbours += 1;
                    }
                    if image.pixel_at(x + 1, y + 0).unwrap() == settings.line_color {
                        neighbours += 1;
                    }
                    if image.pixel_at(x + 1, y + 1).unwrap() == settings.line_color {
                        neighbours += 1;
                    }

                    if neighbours < settings.cleaning_factor {
                        image.set_pixel_at(x, y, settings.background_color).unwrap();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn contours() {
        let heightmap = HeightMap::from_file("image/earth.png").unwrap();
        let contours = generate(&heightmap, Settings::default());
        contours.write_to_file("image/contours.out.png").unwrap();
    }
}
