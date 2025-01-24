use config::{Config, ConfigError, File, Environment};
use super::dto::Settings;

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("settings"))
            .add_source(Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }
}
