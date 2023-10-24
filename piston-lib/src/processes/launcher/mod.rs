use std::error::Error;
use std::path::PathBuf;
use crate::processes::network::{download_from_uri, DownloadResult};

pub mod args;

pub async fn get_pfp_from_uuid(uuid: String, image_path: PathBuf) -> Result<PathBuf, String> {
    return match download_from_uri(&*format!("https://mc-heads.net/avatar/{}/100/helm.png", uuid), &image_path, None, true).await {
        Err(e) => {
            if image_path.exists() {
                Ok(image_path)
            }
            else {
                Err(e.to_string())
            }
        },
        _ => {
            Ok(image_path)
        }
    }
}