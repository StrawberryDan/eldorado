use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(name = "El Dorado", version = "1.0", author = "DanRhysMorris@gmail.com")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Options {
    #[clap(short, long, default_value="dorado.json", aliases=&["config", "configuration"])]
    pub config_file: String,
}