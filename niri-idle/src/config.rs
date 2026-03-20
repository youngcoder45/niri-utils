use serde::Deserialize;
use std::{env, fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub timeout: Option<u32>,
    pub command: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        // 🔥 get HOME safely
        let home = match env::var("HOME") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Failed to get HOME directory");
                return Self::default();
            }
        };

        let path: PathBuf = PathBuf::from(home)
            .join(".config")
            .join("niri-idle")
            .join("config.toml");

        // 🔥 read config file if exists
        if let Ok(content) = fs::read_to_string(&path) {
            match toml::from_str(&content) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Config parse error: {}", e);
                    Self::default()
                }
            }
        } else {
            // file doesn't exist → use defaults
            Self::default()
        }
    }
}

// =====================
// DEFAULT VALUES
// =====================
impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Some(300_000), // 5 minutes
            command: Some(format!(
                "{}/.config/niri-lock/lock.sh",
                env::var("HOME").unwrap()
            )),
        }
    }
}
