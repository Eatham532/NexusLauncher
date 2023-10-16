// The config works in two ways:
// There is the app config which will store the config with the directory to a custom config.
// This will let the user have more options where things are stored

pub mod structs;

use std::fs;
use std::path::PathBuf;
use tauri::api::path;
use crate::config::structs::config::AppConfig;
use crate::config::structs::instances::InstancesToml;

/// Gets the path of the directory which contains the app configurations
#[tauri::command]
#[specta::specta]
pub fn get_app_config_dir_path() -> PathBuf {
    path::config_dir().unwrap().join("NexusLauncher")
}




/// Gets the path of the app config.toml
fn get_app_config_path() -> PathBuf {
    get_app_config_dir_path().join("config.toml")
}

/// Gets the data from config.toml
#[tauri::command]
#[specta::specta]
pub fn get_app_config() -> AppConfig {
    let p = get_app_config_path();

    if !p.exists() {
        // Create file with default settings
        if let Ok(toml_string) = toml::to_string(&AppConfig::default()) {
            return match fs::write(&p, toml_string) {
                Ok(_) => {
                    println!("Created `config.toml` with default settings!");
                    AppConfig::default()
                }
                Err(e) => {
                    eprintln!("Failed to create `config.toml`! {e}");
                    AppConfig::default()
                }
            };
        }
    }

    match std::fs::read_to_string(p) {
        Ok(v) => toml::from_str(&v).unwrap(),
        Err(_) => AppConfig::default(),
    }
}

/// Write the app config to config.toml
#[tauri::command]
#[specta::specta]
pub fn write_app_config(config: AppConfig) {
    let p = get_app_config_path();

    if let Ok(toml_string) = toml::to_string(&config) {
        match std::fs::write(p, toml_string) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to write config file: {e}");
            }
        }
    }
}



/// Get the path of the instances.toml
fn get_instances_toml_path() -> PathBuf {
    get_app_config_dir_path().join("instances.toml")
}

/// Get the data from instances.toml
#[tauri::command]
#[specta::specta]
pub fn get_instances_toml() -> InstancesToml {
    let p = get_instances_toml_path();

    if !p.exists() {
        // Create file with default settings
        write_instance_toml(InstancesToml::default());
    }

    match std::fs::read_to_string(p) {
        Ok(v) => toml::from_str(&v).unwrap(),
        Err(_) => InstancesToml::default(),
    }
}

/// Write instances data to instances.toml
#[tauri::command]
#[specta::specta]
pub fn write_instance_toml(config: InstancesToml) {
    let p = get_instances_toml_path();

    match toml::to_string(&config) {
        Ok(toml_string) => {
            match std::fs::write(p, toml_string) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to write config file: {e}");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to convert config to Toml: {e}");
        }
    }
}