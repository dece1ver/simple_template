use config::{Config, File};
use eyre::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub general: General,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub log_level: String,
    pub log_type: LogType,
}

#[derive(Debug, Deserialize)]
pub enum LogType {
    FILE,
    STDERR,
    BOTH,
}

pub fn load_config() -> Result<Settings> {
    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()?;
    Ok(config.try_deserialize()?)
}
