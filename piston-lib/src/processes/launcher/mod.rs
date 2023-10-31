use std::error::Error;
use std::path::PathBuf;
use crate::processes::network::{download_from_uri, DownloadResult};

pub mod args;

pub async fn get_pfp_from_uuid(uuid: String, image_path: PathBuf, overwrite: bool) -> Result<PathBuf, String> {
    return match download_from_uri(&*format!("https://crafatar.com/avatars/{}?size=100&default=MHF_Steve&overlay", uuid), &image_path, None, overwrite).await {
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