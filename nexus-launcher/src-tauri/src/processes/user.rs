use piston_lib::processes::launcher::get_pfp_from_uuid;
use crate::config::get_cache_path;

#[tauri::command]
#[specta::specta]
pub async fn get_pfp_path(uuid: String) -> Result<String, String> {
    let path = get_pfp_from_uuid(uuid.clone(), get_cache_path().join(format!("icons\\mc-heads\\{}.png", uuid))).await;

    return match path {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => {
            println!("{}", e);
            Ok(String::new())
        }
    }
}