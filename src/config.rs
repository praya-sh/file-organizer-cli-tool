use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub categories: HashMap<String, Vec<String>>,
}

impl Config {
    /// Returns the default categories (matches the original hardcoded values).
    pub fn default_config() -> Self {
        let mut categories = HashMap::new();
        categories.insert(
            "Images".to_string(),
            vec!["png", "jpg", "jpeg", "gif", "bmp", "svg", "webp"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        categories.insert(
            "Docs".to_string(),
            vec!["pdf", "doc", "docx", "txt", "rtf", "otd"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        categories.insert(
            "Videos".to_string(),
            vec!["mp4", "mkv", "avi", "mov", "wmv", "flv"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        categories.insert(
            "Audio".to_string(),
            vec!["mp3", "wav", "flac", "aac", "ogg", "m4a"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        categories.insert(
            "Archives".to_string(),
            vec!["zip", "rar", "7z", "tar", "gz", "bz2"]
                .into_iter()
                .map(String::from)
                .collect(),
        );
        Config { categories }
    }

    /// Returns the global config file path: <config_dir>/tidy/config.toml
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("tidy").join("config.toml"))
    }

    /// Loads config from the global config file.
    /// If no file exists, returns the hardcoded defaults.
    /// If a file exists, merges user categories on top of defaults.
    pub fn load() -> Self {
        let default = Self::default_config();

        let path = match Self::config_path() {
            Some(p) => p,
            None => return default,
        };

        if !path.exists() {
            return default;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: Could not read config file {:?}: {}", path, e);
                return default;
            }
        };

        let user_config: Config = match toml::from_str(&content) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: Could not parse config file {:?}: {}", path, e);
                return default;
            }
        };

        // Merge: start with defaults, then apply user overrides
        let mut merged = default.categories;
        for (category, extensions) in user_config.categories {
            merged.insert(category, extensions);
        }

        Config { categories: merged }
    }

    /// Saves the default config to the global config path.
    /// Used by --init-config.
    pub fn save_default() -> Result<PathBuf, String> {
        let path = Self::config_path().ok_or("Could not determine config directory")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let default = Self::default_config();
        let content = toml::to_string_pretty(&default)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, content).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(path)
    }
}
