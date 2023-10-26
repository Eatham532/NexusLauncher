use piston_lib::processes::launcher::get_pfp_from_uuid;
use crate::config::{get_cache_path, get_users, write_users_json};

#[tauri::command]
#[specta::specta]
pub async fn get_pfp_path(uuid: String) -> Result<String, String> {
    let path = get_pfp_from_uuid(uuid.clone(), get_cache_path().join(format!("icons\\mc-heads\\{}.png", uuid))).await;

    return match path {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => {
            println!("{}", e);
            Ok("/ProfileIconPlaceholder.png".to_string())
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn change_active_user(uuid: String) {
    let mut users = get_users();
    if users.users.contains_key(&uuid) {
        users.active = Some(uuid);
    }

    write_users_json(users);
}

#[tauri::command]
#[specta::specta]
pub fn logout_user(uuid: String) {
    let mut users = get_users();
    if users.users.contains_key(&uuid) {
        users.users.remove(&uuid);
    }

    if users.active == Some(uuid) {
        users.active = None;
    }

    write_users_json(users);
}