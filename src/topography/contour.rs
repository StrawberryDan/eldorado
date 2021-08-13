use super::*;

#[derive(Clone, Copy)]
pub struct Settings {
    pub line_divisions: u16,
    pub line_color: Color,
    pub background_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            line_color: Color::from([1.0, 0.0, 0.0]),
            background_color: Color::from([0.0, 0.0, 0.0, 0.0]),
            line_divisions: 32,
        }
    }
}

pub fn generate(heightmap: &HeightMap, settings: Settings) -> Image {
    let mut contours = Image::new(heightmap.width(), heightmap.height()).fill(settings.background_color);

    let mut leveled = heightmap.clone();
    *leveled.data_mut() = leveled.data().iter().map(|v| v / (u16::MAX / settings.line_divisions)).collect();
   
    for x in 0..heightmap.width() {
        for y in 0..heightmap.height() {
            let v = leveled.height_at(x, y).unwrap();
            let neighbours = leveled.orthoganal_neighbours(x, y);
            let count = neighbours.iter().filter(|(_, nv)| v > *nv).count();

            if count > 0 && count < 8 {
                contours.set_pixel_at(x, y, settings.line_color).unwrap();
            }
        }
    }
    

    return contours;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contours() {
        let image = Image::from_file("image/earth.png").unwrap();

        let heightmap = HeightMap::from(image);

        let contours = generate(&heightmap, Settings::default());
        contours.write_to_file("image/contours.out.png").unwrap();
    }
}
