use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Type)]
pub struct AppConfig {
    pub metadata_dir: String,
    pub cache_dir: String,
    pub default_instances_dir: String,
    pub dev_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            metadata_dir: crate::config::get_appdata_dir_path().join("meta").display().to_string(),
            cache_dir: crate::config::get_appdata_dir_path().join("cache").display().to_string(),
            default_instances_dir: crate::config::get_appdata_dir_path().join("instances").display().to_string(),
            dev_mode: false,
        }
    }
}