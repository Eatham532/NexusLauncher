use std::fs::read_to_string;
use std::path::PathBuf;
use serde::forward_to_deserialize_any;
use crate::data_structures::game::metadata::piston_version_manifest::PistonMetadata;
use crate::data_structures::game::modded::Modloader;
use crate::data_structures::game::version::game_version;
use crate::processes::launcher::installation::vanilla::{download_asset_objects, download_client_jar, download_libraries, get_version_info};
use crate::processes::network::download_from_uri;

pub mod vanilla;
pub mod versioning;
pub mod modded;
mod forge;

pub trait HandleProgress: Send + Sync {
    fn update_progress(&self, progress: i32, id: &str, message: &str);
}

pub async fn install(version_metadata: PistonMetadata, version_id: &str, modloader: &Modloader, loader_id: &Option<String>, instances_dir: &PathBuf, data_dir: &PathBuf, progress_handler: &dyn HandleProgress) {
    let versions_dir = data_dir.join("versions");

    progress_handler.update_progress(10, "1", "Grabbing client.json");
    match modloader {
        Modloader::Vanilla => {
            vanilla::download_client_json(&version_metadata, version_id, &versions_dir).await.unwrap();
        },
        Modloader::Fabric | Modloader::Quilt => {
            // Download fabric json
            modded::download_fabric_json(&version_metadata, version_id, modloader, loader_id.as_ref().unwrap(), &versions_dir).await;
        },
        Modloader::Forge | Modloader::NeoForge  => {
            forge::do_forge_work(&version_metadata, &version_id, modloader, loader_id.as_ref().unwrap(), &data_dir).await;
        },
        _ => {
            // Other. The user is using their own installation.
            panic!("Unsupported");
        }
    };

    let id = if loader_id.is_some() {format!("{}-loader-{}-{}", modloader, loader_id.clone().unwrap(), version_id)} else {version_id.to_owned()};
    println!("Id: {}", &id);
    let version_info = get_version_info(&id, &versions_dir).await.unwrap();
    install_from_version_info(version_info, data_dir, progress_handler).await.unwrap();
    progress_handler.update_progress(99, "1", "Wrapping Up");
    progress_handler.update_progress(100, "1", "Done");
}

pub async fn install_from_version_info(version_info: game_version, data_dir: &PathBuf, progress: &dyn HandleProgress) -> Result<(), String> {
    println!("{:?}", &version_info.id);
    download_client_jar(&version_info, &data_dir.join("versions")).await.unwrap();

    // Download asset index
    println!("Downloading asset index...");
    let assets_dir = data_dir.join("assets");
    let asset_index_path = &assets_dir.join("indexes").join(format!("{}.json", version_info.asset_index.id));
    let _ = download_from_uri(&version_info.asset_index.url, asset_index_path, Some(&*version_info.asset_index.sha1), false).await;

    if asset_index_path.exists() {
        println!("Downloading assets");

        let contents = read_to_string(asset_index_path).unwrap();
        let asset_index: vanilla::AssetIndex = serde_json::from_str(&contents).unwrap();

        download_asset_objects(asset_index, &assets_dir).await;
    }
    else {
        return Err("No asset index found!".to_string());
    }

    // Download libraries
    let natives_dir = &data_dir.join(format!("natives/{}", &version_info.id));
    let library_dir = &data_dir.join("libraries");

    download_libraries(library_dir, natives_dir, (&*version_info.libraries).to_vec()).await;

    // Download log_config.xml

    if let Some(logging) = version_info.logging.client {
        download_from_uri(logging.file.url.as_str(),   &data_dir.join(format!("assets/log-configs/{}", logging.file.id)), Some(logging.file.sha1.as_str()), false).await.unwrap();
    }

    println!("Finished installation of {}", &version_info.id);
    Ok(())
}

pub fn get_version_jar_path(data_dir: &PathBuf, game_id: &str) -> PathBuf{
    data_dir.join("versions").join(game_id).join(format!("{}.jar", game_id))
}