use json::{JsonValue};

use crate::color::Color;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Read;

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

        let mut configuration = Configuration{ mapping: HashMap::new() };

        for (key, settings) in json.entries() {
            let key_color = match Color::from_str(key) {
                Ok(c) => c,
                Err(_) => return Err(format!("Unable to parse {} into a color!", key)),
            };

            let color = match &settings["color"] {
                JsonValue::String(s) => {
                    Color::from_str(s.as_str())?
                },

                JsonValue::Null => key_color,

                _ => return Err(String::from("Non color given as biome color")),
            };

            let outline_color= match &settings["outline_color"] {
                JsonValue::String(s) => Color::from_str(s.as_str())?,
                JsonValue::Null => Color::from([0, 0, 0, 255]),
                _ => return Err(String::from("Non color given as biome outline color")),
            };

            let outline_thickness = match &settings["outline_thickness"] {
                JsonValue::Number(n) => match JsonValue::Number(*n).as_u64() {
                    Some(n) =>n,
                    None => return Err(String::from("Value given to outline_thickness is not an insigned integer")),
                },
                JsonValue::Null => 1,
                _ => return Err(String::from("Non number given as outline thickness")),
            };

            configuration.mapping.insert(key_color, BiomeSettings{
                color, outline_color, outline_thickness
            });
        }

        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use crate::biome::Configuration;

    #[test]
    fn read_configuration() {
        let configuration = Configuration::from_file("samples/biomes.json").unwrap();
    }
}