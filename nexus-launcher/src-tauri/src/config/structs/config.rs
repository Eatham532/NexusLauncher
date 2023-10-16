use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Deserialize, Serialize, Type)]
pub struct AppConfig {
    pub metadata_dir: String,
    pub default_instances_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            metadata_dir: crate::config::get_app_config_dir_path().join("meta").display().to_string(),
            default_instances_dir: crate::config::get_app_config_dir_path().join("instances").display().to_string(),
        }
    }
}