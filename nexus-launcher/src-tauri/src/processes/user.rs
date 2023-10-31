use piston_lib::processes::launcher::get_pfp_from_uuid;
use crate::config::{get_cache_path, get_users, write_users_json};

#[tauri::command]
#[specta::specta]
pub async fn get_pfp_path(uuid: String, replace_old: bool) -> Result<String, String> {
    let path = get_pfp_from_uuid(uuid.clone(), get_cache_path().join(format!("icons\\mc-heads\\{}.png", uuid)), replace_old).await;

    return match path {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => {
            println!("{}", e);
            Ok("/ProfileIconPlaceholder.png".to_string())
        }
    }
}

pub fn pre_download_user_icons() {
    let users = get_users();
    for user in users.users {
        let _ = get_pfp_path(user.0, true);
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