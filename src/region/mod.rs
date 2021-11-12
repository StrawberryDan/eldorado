use json::{JsonValue};

use crate::color::Color;
use std::ffi::OsStr;
use std::io::Read;
use crate::image::Image;
use std::collections::HashMap;
use std::path::{PathBuf};

mod glyphs;

struct BiomeSettings {
    color: Color,
    outline_color: Color,
    outline_thickness: u64,
    glyph_image: Option<PathBuf>,
    glyph_density: usize,
    glyph_threshold: usize,
}

pub struct Configuration {
    glyphs: HashMap<PathBuf, Image>,
    mapping: Vec<(Color, BiomeSettings)>,
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

        let mut configuration = Configuration { glyphs: HashMap::new(), mapping: Vec::new() };

        for (key, settings) in json.entries() {
            let key_color = match Color::from_str(key) {
                Ok(c) => c,
                Err(_) => return Err(format!("Unable to parse {} into a color!", key)),
            };

            let color = match &settings["color"].as_str() {
                Some(s) => {
                    Color::from_str(s)?
                }

                None if settings.has_key("color") => return Err(format!("Cannot parse {} as region color", &settings["color"])),
                _ => key_color,
            };

            let outline_color = match &settings["outline_color"].as_str() {
                Some(s) => Color::from_str(s)?,
                None if settings.has_key("outline_color") => return Err(String::from("Non color string given as region outline color")),
                _ => Color::from([0, 0, 0, 255]),
            };

            let outline_thickness = match &settings["outline_thickness"].as_u64() {
                Some(n) => *n,
                None if settings.has_key("outline_thickness") => return Err(format!("Could not parse outline_thickness for {}", key)),
                _ => 0,
            };

            let glyph_image = match settings["glyph_image"].as_str() {
                Some(s) => {
                    let as_path = PathBuf::from(s);
                    if !configuration.glyphs.contains_key(&as_path) {
                        if !as_path.exists() {
                            return Err(format!("Image file: {} does not exist", as_path.to_str().unwrap()));
                        }

                        let image = match Image::from_file(s) {
                            Ok(i) => i,
                            Err(m) => return Err(format!("Could not load glyph {} because {}", as_path.to_str().unwrap(), m)),
                        };

                        configuration.glyphs.insert(as_path.clone(), image);
                    }

                    Some(as_path)
                }
                None if settings.has_key("glyph_image") => return Err(String::from("Non string/filepath given for glyph image")),
                _ => None,
            };

            let glyph_density = match settings["glyph_density"].as_u64() {
                Some(v) => v as usize,
                None if settings.has_key("glyph_density") => return Err(String::from("Invalid data for glyph density")),
                _ => 1,
            };

            let glyph_threshold = match settings["glyph_threshold"].as_u64() {
                Some(v) => v as usize,
                None if settings.has_key("glyph_threshold") => return Err(String::from("Invalid data for glyph density")),
                _ => 100,
            };

            configuration.mapping.push((key_color, BiomeSettings {
                color,
                outline_color,
                outline_thickness,
                glyph_image,
                glyph_density,
                glyph_threshold,
            }));
        }

        configuration.mapping.sort_by(|(_, s1), (_, s2)| s1.outline_thickness.cmp(&s2.outline_thickness));

        return Ok(configuration);
    }

    pub fn generate_layer(&self, biome_map: &Image) -> Result<Image, String> {
        let width = biome_map.width();
        let height = biome_map.height();
        let mut layer = Image::new(width, height);
        let mut glyph_layer = Image::new(width, height);

        for entry in &self.mapping {
            let mut biome = Image::new(width, height).fill(Color::from([0, 0, 0, 0]));

            for x in 0..width {
                for y in 0..height {
                    if biome_map.pixel_at(x, y).unwrap() == entry.0 {
                        biome.set_pixel_at(x, y, entry.1.color)?;
                    }
                }
            }

            for _i in 0..entry.1.outline_thickness {
                let mut outlined = biome.clone();

                for x in 0..width {
                    for y in 0..height {
                        // Check if neighbour is region color or outline color (and must also be itself region color)
                        if biome.pixel_at(x, y).unwrap() != entry.1.color { continue; }
                        let neighbours = biome.get_neighbouring_pixels(x as isize, y as isize)
                            .iter().filter(|(_, c)| *c == Color::from([0, 0, 0, 0]) || *c == entry.1.outline_color)
                            .count();

                        if neighbours > 0 {
                            outlined.set_pixel_at(x, y, entry.1.outline_color)?;
                        }
                    }
                }

                biome = outlined;
            }

            if entry.1.glyph_image.is_some() {
                let glyph = entry.1.glyph_image.clone().unwrap();
                let glyph = self.glyphs.get(&glyph).unwrap();

                let distrib = glyphs::GlyphDistribution::new([52; 32], entry.1.glyph_density, entry.1.glyph_threshold, glyph, &biome_map, entry.0);
                let layer = distrib.to_layer();

                glyph_layer.overlay(&layer).unwrap();
            }

            layer.overlay(&biome)?;
        }

        layer.overlay(&glyph_layer).unwrap();

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

        let biome_map = configuration.generate_layer(&map).unwrap();

        biome_map.write_to_file("../../samples/region_map.png").unwrap();
    }
}