use std::{env, fs, io};

use serde::de::DeserializeOwned;

// TODO トレイト名とかconfigじゃなくてsettingsに直す
pub trait ConfigLoader {
    fn load_config<T>() -> io::Result<T>
    where
        T: DeserializeOwned,
    {
        let settings_path =
            env::var("KOCLI_SETTINGS").expect("KOCLI_SETTINGS must be set");
        let content = fs::read_to_string(&settings_path)?;
        let config: T = toml::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(config)
    }
}
