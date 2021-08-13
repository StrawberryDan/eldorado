use super::*;

pub struct Settings {
    line_divisions: u16,
    light_color: Color,
    dark_color: Color,
    background_color: Color,
    light_dir: Vector<2>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            line_divisions: 32,
            light_color: Color::from([1.0, 1.0, 1.0]),
            dark_color: Color::from([0.0, 0.0, 0.0]),
            background_color: Color::from([0.0, 0.0, 0.0, 0.0]),
            light_dir: Vector::from([1.0, 1.0]),
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
                let normal = heightmap.normal_at(x, y);
                
                let light = match normal {
                    Some(normal) => Vector::dot(settings.light_dir.normalise(), normal),
                    None => continue,
                };

                let c = if light < 0.0 {
                    Color::mix(settings.background_color, settings.light_color, -light)
                } else {
                    Color::mix(settings.background_color, settings.dark_color, light)
                };


                contours.set_pixel_at(x, y, c).unwrap();
            }
        }
    }

    return contours;
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn tanaka() {
        let image = Image::from_file("image/earth.png").unwrap();
        let heightmap = HeightMap::from(image);

        let contours = generate(&heightmap, Settings::default());

        contours.write_to_file("image/tanaka_contours.out.png").unwrap();
    }
}
