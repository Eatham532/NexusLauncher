mod config;

use std::cell::Cell;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use specta::{specta, Type};
use serde::{Deserialize, Serialize};
use tauri::{Window};
use uuid::{Uuid, Context};
use piston_lib::data_structures::game::modded::Modloader;
use piston_lib::data_structures::game::version::VersionType;
use piston_lib::processes::api::mojang::user::User;
use piston_lib::processes::installation;
use piston_lib::processes::installation::{HandleProgress, vanilla};
use piston_lib::processes::installation::vanilla::get_version_info;
use piston_lib::processes::launcher::args::get_classpaths;
use crate::config::{get_app_config, get_appdata_dir_path, get_instances_toml, get_users, write_instance_toml};
use crate::config::user::NexusUser;
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

    fn get_parsed_version_str(&self) -> String {
        match self.modloader {
            Modloader::Vanilla => {
                self.game_version.clone()
            }
            _ => {
                format!("{}-{}", self.game_version, self.loader_version.clone().unwrap())
            }
        }
    }
    
    pub async fn launch(&self) {
        let config = get_app_config();
        let data_dir = PathBuf::from(&config.metadata_dir);

        let versions_dir = PathBuf::from(&config.metadata_dir).join("versions");

        let version_info = get_version_info(&*self.game_version, &versions_dir).await.unwrap();
        let client_jar_path = versions_dir.join(self.get_parsed_version_str()).join(format!("{}.jar", &self.get_parsed_version_str()));
        println!("Client Jar Path: {:?}", client_jar_path);

        let users = get_users();
        let current_user : &NexusUser = match users.active {
            Some(u) => users.users.get(&u).unwrap(),
            None => users.users.values().next().unwrap(),
        };


        let mc_args = piston_lib::processes::launcher::args::MinecraftArgs {
            access_token: current_user.access_token.clone(),
            username: current_user.username.clone(),
            uuid: current_user.uuid.clone(),
            version: self.game_version.clone(),

            asset_index_name: version_info.asset_index.id.to_string(),
            game_directory: PathBuf::from(self.path.clone()),
            assets_directory: data_dir.join("assets"),
            version_type: VersionType::Release.clone(),
            resolution: Default::default(),
        };

        let jvm_args = piston_lib::processes::launcher::args::JvmArgs {
            natives_path: data_dir.join("natives").join(self.game_version.clone()),
            libraries_path: data_dir.join("libraries"),
            class_paths: format!("\"{}\"", get_classpaths(&version_info.libraries, client_jar_path, data_dir.join("libraries"), "windows")),
            version_name: self.game_version.clone(),
            java_arch: "windows".to_string(),
        };

        std::fs::create_dir_all(self.path.clone()).unwrap();

        //let mut command = Command::new("C:\\Users\\eatha\\AppData\\Roaming\\com.modrinth.theseus\\meta\\java_versions\\zulu8.72.0.17-ca-jre8.0.382-win_x64\\bin\\javaw.exe");
        let mut command = Command::new("java");
        if let Some(default_args) = version_info.arguments {
            let args = piston_lib::processes::launcher::args::format_arguments(default_args, mc_args, &jvm_args);
            command.args(args);
        }
        else {
            println!("ERR! No arguments found");
        }

        command.current_dir(self.path.clone()).env_remove("_JAVA_OPTIONS");
        println!("Command: java {}", command_to_string(&command));
        let result = command.spawn().unwrap();
        println!();
        println!("Result:");
        println!("{:?}", result);
    }
}

fn command_to_string(mut command: &Command) -> String {
    let mut args = Vec::new();

    for arg in command.get_args() {
        args.push(arg.to_string_lossy().to_string());
    }

    // Escape any special characters in the arguments.
    for arg in &mut args {
        let _ = arg.escape_default();
    }

    // Join the arguments together with a space.
    args.join(" ")
}