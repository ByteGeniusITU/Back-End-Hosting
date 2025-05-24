use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub address: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let configuration = Config::builder()
        .add_source(File::with_name("configuration"))
        .build()
        .expect("Failed to build configuration");

    configuration.try_deserialize()
}
