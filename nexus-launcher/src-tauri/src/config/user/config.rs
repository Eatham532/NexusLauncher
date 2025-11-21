use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use specta::Type;
use crate::config::user::NexusUser;

#[derive(Serialize, Deserialize, Type)]
pub struct UsersJson {
    pub active: Option<String>,
    pub users: HashMap<String, NexusUser>,
}

impl Default for UsersJson {
    fn default() -> Self {
        UsersJson {
            active: None,
            users: HashMap::new(),
        }
    }
}