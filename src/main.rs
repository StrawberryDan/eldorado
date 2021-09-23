extern crate png;
extern crate rand;
extern crate json;
extern crate clap;

use std::io::Read;
use json::Error;
use crate::image::Image;
use crate::topography::HeightMap;

pub mod color;
pub mod image;
pub mod math;
pub mod topography;
pub mod vector;
pub mod region;
pub mod cli;

fn main() {
    use clap::Clap;

    let options = cli::Options::parse();
    let configuration = get_configuration_file(&options)
        .success_message("Configuration Loaded")
        .print_error()
        .unwrap();

    let regions = &configuration["regions"];

    for region in regions.entries() {
        let map = Image::from_file(region.0).print_error().unwrap();

        let region_config = region::Configuration::from_json(region.1)
            .success_message("Loaded Region Config!")
            .print_error().unwrap();

        let region_layer = region_config.generate_layer(&map).print_error().unwrap();

        region_layer.write_to_file("samples/regions.out.png").unwrap();
    }

    for topography in configuration["topography"].entries() {
        let map = HeightMap::from_file(topography.0).print_error().unwrap();

        let topography_type = topography.1["type"].as_str()
            .expect(format!("Topography type not set for {}", topography.0).as_str());

        match topography_type {
            "tanaka" => {
                let layer = topography::generate_tanaka_layer(&map, topography::TanakaSettings::default());
                layer.write_to_file("samples/topography.out.png").unwrap();
            }

            "shaded" => {
                let layer = topography::generate_shaded_layer(&map, topography::ShadedSettings::default());
                layer.write_to_file("samples/topography.out.png").unwrap();
            }

            _ => {
                eprintln!("Invalid topography type: {}", &topography_type);
                return;
            }
        }
    }
}

fn get_configuration_file(options: &cli::Options) -> Result<json::JsonValue, String> {
    {
        let string = match std::fs::File::open(&options.config_file) {
            Ok(mut f) => {
                let mut string = String::new();
                f.read_to_string(&mut string);
                string
            }

            Err(_) => {
                return Err(format!("Configuration file: {} could not be openned!", &options.config_file));
            }
        };

        match json::parse(&*string) {
            Ok(j) => {
                return Ok(j);
            }

            Err(e) => match e {
                Error::UnexpectedCharacter { ch, line, column } => {
                    return Err(format!("[{}] Unexpected character \'{}\' at {}:{}!", &options.config_file, ch, line, column));
                }

                Error::UnexpectedEndOfJson => {
                    return Err(format!("[{}] JSON file ended unexpectedly!", &options.config_file));
                }

                Error::ExceededDepthLimit => {
                    return Err(format!("[{}] JSON file depth limit exceeded!", &options.config_file));
                }

                Error::FailedUtf8Parsing => {
                    return Err(format!("[{}] Failed to parse file as UTF-8!", &options.config_file));
                }

                Error::WrongType(message) => {
                    return Err(format!("[{}] Wrong Type Error ({})!", &options.config_file, message));
                }
            }
        }
    };
}

trait ReportSuccess {
    fn success_message(self, message: &'static str) -> Self;
}

impl<T> ReportSuccess for Result<T, String> {
    fn success_message(self, message: &'static str) -> Self {
        self.map(|v| {
            println!("{}", message);
            return v;
        })
    }
}

trait ErrorMessage<T> {
    fn print_error(self) -> Self;
}

impl<T, S: ToString> ErrorMessage<T> for Result<T, S> {
    fn print_error(self) -> Result<T, S> {
        self.map_err(|e| {
            eprintln!("{}", e.to_string());
            return e;
        })
    }
}
