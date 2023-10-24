use tauri::{State, Window};
use piston_lib::data_structures::game::mojang_version_manifest::VersionManifestRoot;
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
/// Returns a list of versions from Mojang's version manifest
pub async fn get_versions() -> VersionManifestRoot {
    let versions = piston_lib::processes::installation::vanilla::get_version_manifest().await.unwrap();
    versions
}

#[tauri::command]
#[specta::specta]
/// Launch an instance
pub async fn launch_instance(instance: NexusInstance) {
    instance.launch().await;
    println!("Done");
}