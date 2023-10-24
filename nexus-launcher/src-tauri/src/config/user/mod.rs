use serde::{Deserialize, Serialize};
use std::time::Instant;
use specta::Type;

pub mod config;

#[derive(Serialize, Deserialize, Type)]
pub struct NexusUser {
    pub uuid: String,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires: String,
}

impl NexusUser {
    pub fn new(uuid: String, username: String, access_token: String, refresh_token: String, expires: String) -> Self {
        NexusUser {
            uuid,
            username,
            access_token,
            refresh_token,
            expires,
        }
    }
}
