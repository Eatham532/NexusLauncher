use std::path::PathBuf;
use crate::data_structures::game::modded::Modloader;
use crate::data_structures::game::version::game_version;
use crate::processes::installation::vanilla::{download_client_json, get_version_info};

pub mod vanilla;

pub trait HandleProgress: Send + Sync {
    fn update_progress(&self, progress: i32, id: &str, message: &str);
}

pub async fn install(version_id: &str, modloader: &Modloader, instances_dir: &PathBuf, data_dir: &PathBuf, progress_handler: &dyn HandleProgress) {
    let versions_dir = data_dir.join("versions");

    match modloader {
        Modloader::Vanilla => {
            download_client_json(version_id, &versions_dir).await.unwrap();
            progress_handler.update_progress(10, "1", "Downloading client.json");
            let version_info = get_version_info(version_id, &versions_dir).await.unwrap();
            version_info.install(data_dir, progress_handler).await.unwrap();
            progress_handler.update_progress(99, "1", "Wrapping Up");
            progress_handler.update_progress(100, "1", "Done");
        },
        _ => {}
    }
}