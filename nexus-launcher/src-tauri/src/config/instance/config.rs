use std::path::{PathBuf};
use std::process::abort;
use piston_lib::data_structures::client::game_profile::ProfileOptions;
use crate::config::instance::NexusInstance;
use crate::config::write_instance_toml;

impl NexusInstance {
    fn get_instance_options(&self) {
        let p = PathBuf::from(&self.path).join("profile.toml");

        if !p.exists() {
            self.write_instance_options(ProfileOptions::new(8, 8));
        }
    }

    fn write_instance_options(&self, config: ProfileOptions) {
        let p = PathBuf::from(&self.path).join("profile.toml");

        match toml::to_string(&config) {
            Ok(toml_string) => {
                match std::fs::write(p, toml_string) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to write profile.toml file: {e}");
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to convert config to Toml: {e}");
            }
        }
    }
}

