use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use piston_lib::data_structures::game::metadata::piston_version_manifest::PistonMetadata;
use piston_lib::data_structures::game::modded;
use piston_lib::data_structures::game::version::game_version;
use piston_lib::processes::launcher::args::{format_arguments, get_classpaths};
use piston_lib::processes::launcher::installation;
use piston_lib::processes::launcher::installation::install_from_version_info;
use piston_lib::processes::launcher::installation::modded::merge_partial_version;
use crate::{ID, InstallProgressHandler, LOADER, LOADER_ID};


pub fn install() {
    let data_dir: PathBuf = PathBuf::from_str("C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\cache\\game_data").unwrap();

    let json = std::fs::read_to_string("C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\cache\\version_metadata.json").unwrap();
    let versions = serde_json::from_str::<PistonMetadata>(json.as_str()).unwrap();
    let version_info = versions.versions.iter().find(|p| p.id.as_str() == ID).unwrap();

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        installation::install(versions, ID, LOADER, &Some(LOADER_ID.to_owned()), &data_dir, &data_dir, &InstallProgressHandler::new()).await;
    });
}

pub fn launch() {
    let data_dir: PathBuf = PathBuf::from_str("C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\cache\\game_data").unwrap();

    let json = std::fs::read_to_string("C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\cache\\game_data\\versions\\forge-loader-49.0.12-1.20.4\\forge-loader-49.0.12-1.20.4.json").unwrap();
    let version_info = serde_json::from_str::<game_version>(json.as_str()).unwrap();
    let id = format!("{}-loader-{}-{}", LOADER, LOADER_ID, ID);
    let game_data_path = data_dir.join("game_data");
    std::fs::create_dir_all(&game_data_path).unwrap();

    println!("Generating args");
    let mc_args = piston_lib::processes::launcher::args::MinecraftArgs {
        access_token: "84471741f0e84993864b19f8b82d2329".to_string(),
        username: "Eatham532".to_string(),
        uuid: "43d243250ac2454ba70862ebd28559de".to_string(),
        version: ID.to_string(),

        asset_index_name: version_info.asset_index.id.to_string(),
        game_directory: game_data_path.clone(),
        assets_directory: data_dir.clone().join("assets"),
        version_type: version_info.type_,
        resolution: Default::default(),
    };

    println!("Mc args");

    println!("{}", game_data_path.clone().to_str().unwrap());

    let jvm_args = piston_lib::processes::launcher::args::JvmArgs {
        natives_path: data_dir.join("natives").join(id.clone()),
        libraries_path: data_dir.join("libraries"),
        class_paths: format!("{}", get_classpaths(&version_info.libraries, data_dir.join("versions").join(id.clone()).join(format!("{}.jar", id.clone())), data_dir.join("libraries"))),
        version_name: id.clone(),
        log_config_arg: match version_info.logging.client {
            Some(c) => {
                c.argument.replace("${path}", data_dir.join(format!("assets/log-configs/{}", c.file.id)).to_str().unwrap())
            },
            None => {"".to_string()}
        },
        min_mem: 1024,
        max_mem: 4096
    };

    println!("Creating command");
    //let mut command = Command::new("C:\\Users\\eatha\\AppData\\Roaming\\com.modrinth.theseus\\meta\\java_versions\\zulu8.72.0.17-ca-jre8.0.382-win_x64\\bin\\javaw.exe");
    let mut command = Command::new("java");

    command.args(format_arguments(version_info.arguments, version_info.minecraft_arguments, mc_args, jvm_args, version_info.main_class));

    command.current_dir(&game_data_path).env_remove("_JAVA_OPTIONS");

    println!("Command: java {:?}", &command);
    let result = command
        .spawn().unwrap();
}