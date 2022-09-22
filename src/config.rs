use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
}

#[derive(Debug, Clone)]
pub struct NoConfigError;

macro_rules! ENV_FILE_PATH { () => { concat!(env!("CARGO_MANIFEST_DIR"), "/.env") } }

fn parse_from_embedded_file() -> serde_json::Result<Config> {
    let embedded_config = include_str!(ENV_FILE_PATH!());
    serde_json::from_str::<Config>(embedded_config)

}

impl Config {

    // Config will try to initialize from ENV_FILE_PATH.  This should be a proper JSON file,
    // parsable to Config struct.
    pub fn init() -> Result<Self, NoConfigError> {
        if let Ok(config) = parse_from_embedded_file() {
            return Ok(config);
        }
        Err(NoConfigError)
    }
}
