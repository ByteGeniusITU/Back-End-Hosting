use config::{Config};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub address: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let _configuration_directory = base_path.join("configuration");

    let _environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());

    let configuration = Config::builder()
        //.add_source(File::from(configuration_directory.join(environment)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .expect("Failed to build configuration");

    configuration.try_deserialize()
}
