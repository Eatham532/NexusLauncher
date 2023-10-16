use std::cell::Cell;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Mutex;
use specta::{specta, Type};
use serde::{Deserialize, Serialize};
use tauri::{command, Window};
use uuid::{Uuid, Context};
use piston_lib::data_structures::game::modded::Modloader;
use piston_lib::processes::installation;
use piston_lib::processes::installation::{HandleProgress, vanilla};
use crate::config::{get_app_config, get_app_config_dir_path, get_instances_toml, write_instance_toml};
use crate::handler::install_progress_handler::InstallProgressHandler;

#[derive(Deserialize, Serialize, Type)]
pub struct InstancesToml {
    #[serde(rename = "Instance")]
    pub instances: Vec<NexusInstance>,
}


impl Default for InstancesToml {
    fn default() -> Self {
        Self {
            instances: Vec::new(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct NexusInstance {
    pub id: String,
    pub install_stage: InstanceInstallStage,
    pub name: String,
    pub game_version: String,
    pub modloader: Modloader,
    pub loader_version: Option<String>,
    pub path: String,
}

impl Default for NexusInstance {
    fn default() -> Self {
        Self {
            id: Uuid::now_v1(&[1, 2, 3, 4, 5, 6]).to_string(),
            install_stage: InstanceInstallStage::None,
            name: "New Instance".to_string(),
            game_version: "1.20".to_string(),
            modloader: Modloader::Vanilla,
            loader_version: None,
            path: "./".to_string(),
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
pub enum InstanceInstallStage {
    Installed,
    Installing,
    Cancelled,
    None,
}


impl NexusInstance {
    pub async fn install(&mut self, window: Window) {
        /// TODO: Add an Id system to identify the instance in the toml file

        let progress = InstallProgressHandler::new(window);

        self.install_stage = InstanceInstallStage::Installing;
        // 1%
        progress.update_progress(1, &self.id, "Updating Instances Toml");
        self.update_toml();



        let data_dir = PathBuf::from(get_app_config().metadata_dir);
        installation::install(&self.game_version, &self.modloader, &PathBuf::from(&self.path), &data_dir, &progress).await;

        // 99%
        progress.update_progress(99, &self.id, "Updating Instances Toml");
        self.install_stage = InstanceInstallStage::Installed;
        self.update_toml();
        progress.update_progress(100, &self.id, "Done");
    }

    fn update_toml(&self) {
        let mut instance_toml = get_instances_toml();
        match instance_toml.instances.iter().position(|x| x.id == self.id) {
            Some(index) => {
                instance_toml.instances[index] = self.clone();
                write_instance_toml(instance_toml);
            },
            None => {
                instance_toml.instances.push(self.clone());
                write_instance_toml(instance_toml);
            }
        }

    }
}

