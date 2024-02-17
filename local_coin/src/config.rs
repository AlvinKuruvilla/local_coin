use config::{Config, ConfigError, File, FileFormat};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::RwLock;

#[derive(Debug, Deserialize, Clone)]
pub struct LocalCoinConfig {
    contract_path: String,
}
lazy_static! {
    static ref CONFIG: RwLock<Option<LocalCoinConfig>> = RwLock::new(None);
}
fn get_config() -> Result<LocalCoinConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::new("local_coin_config", FileFormat::Yaml))
        .build()
        .unwrap();
    let settings = config.try_deserialize::<LocalCoinConfig>()?;
    Ok(settings)
}
pub fn contract_path() -> String {
    {
        let config_read = CONFIG.read().unwrap();
        if let Some(conf) = &*config_read {
            return conf.contract_path.clone();
        }
    }

    let config = get_config().unwrap();
    let path = config.contract_path.clone();

    let mut config_write = CONFIG.write().unwrap();
    *config_write = Some(config);

    path
}
