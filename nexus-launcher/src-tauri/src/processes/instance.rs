use tauri::{State, Window};
use piston_lib::data_structures::game::metadata::piston_version_manifest::PistonMetadata;
use piston_lib::processes::launcher::installation::versioning::generate_versions_metadata;
use crate::config::get_cache_path;
use crate::config::instance::NexusInstance;
use crate::services::install_service::InstallationService;


#[tauri::command]
#[specta::specta]
/// Adds an instance to the installation queue
pub async fn install_instance<'a>(service: State<'a, InstallationService>, instance: NexusInstance, window: Window) -> Result<(), ()> {
    println!("Adding an instance to the queue");
    service.add_instance(instance, window).await;
    Ok(())
}

#[tauri::command]
#[specta::specta]
/// Returns a list of versions
pub fn get_versions() -> PistonMetadata {
    let metadata_path = get_cache_path().join("version_metadata.json");
    let data = std::fs::read_to_string(metadata_path).unwrap();
    serde_json::from_str::<PistonMetadata>(data.as_str()).unwrap()
}

#[tauri::command]
#[specta::specta]
/// Launch an instance
pub async fn launch_instance(instance: NexusInstance, window: Window) {
    instance.launch(window).await;
    println!("Done");
}

#[tauri::command]
#[specta::specta]
/// Delete an instance
pub fn delete_instance(instance: NexusInstance) {
    instance.delete();
}