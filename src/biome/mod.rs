use json::{JsonValue};

use crate::color::Color;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Read;
use crate::image::Image;

struct BiomeSettings {
    color: Color,
    outline_color: Color,
    outline_thickness: u64,
}

pub struct Configuration {
    mapping: HashMap<Color, BiomeSettings>,
}

impl Configuration {
    fn from_file(file: impl AsRef<OsStr>) -> Result<Self, String> {
        let mut file = std::fs::File::open(file.as_ref()).map_err(|e| e.to_string())?;
        let mut string = String::new();
        file.read_to_string(&mut string).map_err(|e| e.to_string())?;
        return Self::from_string(string);
    }

    fn from_string(string: impl AsRef<str>) -> Result<Self, String> {
        let json = json::parse(string.as_ref()).map_err(|e| e.to_string())?;
        return Self::from_json(json);
    }

    fn from_json(json: JsonValue) -> Result<Self, String> {
        use std::str::FromStr;

        let mut configuration = Configuration { mapping: HashMap::new() };

        for (key, settings) in json.entries() {
            let key_color = match Color::from_str(key) {
                Ok(c) => c,
                Err(_) => return Err(format!("Unable to parse {} into a color!", key)),
            };

            let color = match &settings["color"].as_str() {
                Some(s) => {
                    Color::from_str(s)?
                }

                _ => return Err(format!("Cannot parse {} as biome color", &settings["color"])),
            };

            let outline_color = match &settings["outline_color"].as_str() {
                Some(s) => Color::from_str(s)?,
                None if settings.has_key("outline_color") => return Err(String::from("Non color string given as biome outline color")),
                _ => Color::from([0, 0, 0, 255]),
            };

            let outline_thickness = match &settings["outline_thickness"].as_u64() {
                Some(n) => *n,
                None if settings.has_key("outline_thickness") => return Err(format!("Could not parse outline_thickness for {}", key)),
                _ => 1,
            };

            configuration.mapping.insert(key_color, BiomeSettings {
                color,
                outline_color,
                outline_thickness,
            });
        }

        return Ok(configuration);
    }

    pub fn generate_layer(&self, biome_map: Image) -> Result<Image, String> {
        let width = biome_map.width();
        let height = biome_map.height();
        let mut layer = Image::new(width, height);

        for entry in &self.mapping {
            let mut biome = Image::new(width, height).fill(Color::from([0, 0, 0, 0]));

            for x in 0..width {
                for y in 0..height {
                    if biome_map.pixel_at(x, y).unwrap() == *entry.0 {
                        biome.set_pixel_at(x, y, entry.1.color)?;
                    }
                }
            }

            for _i in 0..entry.1.outline_thickness {
                let mut outlined = biome.clone();
                for x in 0..width {
                    for y in 0..height {
                        // Check if neighbour is biome color or outline color (But not itself biome color)
                        if biome.pixel_at(x, y).unwrap() == entry.1.color { continue; }
                        let neighbours = biome.get_neighbouring_pixels(x as isize, y as isize)
                            .iter().filter(|(_, c)| *c == entry.1.color || *c == entry.1.outline_color)
                            .count();

                        if neighbours > 0 {
                            outlined.set_pixel_at(x, y, entry.1.outline_color)?;
                        }
                    }
                }

                biome = outlined;
            }

            layer.overlay(&biome)?;
        }

        return Ok(layer);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_configuration() {
        Configuration::from_file("samples/biomes.json").unwrap();
    }

    #[test]
    fn generate_layer() {
        let configuration = Configuration::from_file("samples/biomes.json").unwrap();
        let map = Image::from_file("samples/biomes.png").unwrap();

        let biome_map = configuration.generate_layer(map).unwrap();

        biome_map.write_to_file("samples/biome_map.out.png").unwrap();
    }
}