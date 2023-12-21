mod config;

use std::cell::Cell;
use std::collections::VecDeque;
use std::fs;
use std::fs::{canonicalize, remove_dir_all};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::process::{Command};
use std::sync::Mutex;
use specta::{specta, Type};
use serde::{Deserialize, Serialize};
use tauri::{Window};
use uuid::{Uuid, Context};
use piston_lib::data_structures::game::metadata::piston_version_manifest::PistonMetadata;
use piston_lib::data_structures::game::modded::Modloader;
use piston_lib::data_structures::game::version::VersionType;
use piston_lib::processes::api::mojang::user::User;
use piston_lib::processes::launcher::installation;
use piston_lib::processes::launcher::installation::{HandleProgress, vanilla};
use piston_lib::processes::launcher::installation::vanilla::{get_version_info, get_version_manifest};
use piston_lib::processes::launcher::args::{format_arguments, get_classpaths};
use crate::config::{add_user, get_app_config, get_appdata_dir_path, get_cache_path, get_instances_toml, get_users, write_instance_toml, write_users_json};
use crate::config::user::NexusUser;
use crate::handlers::install_progress_handler::InstallProgressHandler;
use crate::processes::instance::get_versions;

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

        println!("Installing an instance with the following data: {:?}", &self);

        self.install_stage = InstanceInstallStage::Installing;
        // 1%
        progress.update_progress(1, &self.id, "Updating Instances Toml");
        self.update_toml();

        progress.update_progress(1, &self.id, "Reading metadata");
        let metadata = serde_json::from_str::<PistonMetadata>(tokio::fs::read_to_string(get_cache_path().join("version_metadata.json")).await.unwrap().as_str()).unwrap();

        let data_dir = PathBuf::from(get_app_config().metadata_dir);
        installation::install(metadata, &self.game_version, &self.modloader,  &self.loader_version, &PathBuf::from(&self.path), &data_dir, &progress).await;

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
                format!("{}-loader-{}-{}", self.modloader, self.loader_version.as_ref().unwrap(), self.game_version)
            }
        }
    }
    
    pub async fn launch(&self, window: Window) {
        let config = get_app_config();
        let data_dir = PathBuf::from(&config.metadata_dir);

        let versions_dir = PathBuf::from(&config.metadata_dir).join("versions");
        let id = self.get_parsed_version_str();

        let version_info = get_version_info(&id, &versions_dir).await.unwrap();
        let client_jar_path = versions_dir.join(self.get_parsed_version_str()).join(format!("{}.jar", &self.get_parsed_version_str()));
        println!("Client Jar Path: {:?}", client_jar_path);

        let mut users = get_users();
        let mut mut_user = match &users.active {
            Some(u) => users.users.get(u).unwrap().to_owned(),
            None => users.users.values().next().unwrap().to_owned(),
        };

        let progress = InstallProgressHandler::new(window);
        installation::install(get_versions(), &self.game_version, &self.modloader, &self.loader_version, &PathBuf::from(&self.path), &data_dir, &progress).await;

        let current_user = &mut_user.clone();

        println!("Generating args");
        let mc_args = piston_lib::processes::launcher::args::MinecraftArgs {
            access_token: mut_user.get_minecraft_access_token().await.to_string(),
            username: current_user.username.clone(),
            uuid: current_user.uuid.clone(),
            version: self.game_version.clone(),

            asset_index_name: version_info.asset_index.id.to_string(),
            game_directory: PathBuf::from(self.path.clone()),
            assets_directory: data_dir.join("assets"),
            version_type: version_info.type_,
            resolution: Default::default(),
        };

        println!("Mc args");

        let jvm_args = piston_lib::processes::launcher::args::JvmArgs {
            natives_path: data_dir.join("natives").join(&id),
            libraries_path: data_dir.join("libraries"),
            class_paths: format!("{}", get_classpaths(&version_info.libraries, client_jar_path, data_dir.join("libraries"))),
            version_name: self.game_version.clone(),
            log_config_arg: version_info.logging.client.argument.replace("${path}", data_dir.join(format!("assets/log-configs/{}", version_info.logging.client.file.id)).to_str().unwrap()),
        };

        std::fs::create_dir_all(self.path.clone()).unwrap();

        println!("Creating command");
        //let mut command = Command::new("C:\\Users\\eatha\\AppData\\Roaming\\com.modrinth.theseus\\meta\\java_versions\\zulu8.72.0.17-ca-jre8.0.382-win_x64\\bin\\javaw.exe");
        let mut command = Command::new("java");

        command.args(format_arguments(version_info.arguments, version_info.minecraft_arguments, mc_args, jvm_args, version_info.main_class));

        command.current_dir(self.path.clone()).env_remove("_JAVA_OPTIONS");

        println!("Command: java {:?}", &command);
        let result = command
            .spawn().unwrap();
    }

    pub fn delete(&self) {
        let mut instance_toml = get_instances_toml();
        match instance_toml.instances.iter().position(|x| x.id == self.id) {
            Some(index) => {
                instance_toml.instances.remove(index);
                write_instance_toml(instance_toml);
            },
            None => {}
        }
        if (Path::new(&self.path)).exists() {
            remove_dir_all(self.path.clone()).expect("Failed to delete instance data dir");
        }

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