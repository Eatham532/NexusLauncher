use serde::{Deserialize, Serialize};
use std::time::Instant;
use specta::Type;
use tokio::task;
use piston_lib::processes::api::mojang::exchange_microsoft_token;
use piston_lib::processes::auth::{get_auth_client, refresh_auth_token};
use crate::config::add_user;

pub mod config;

#[derive(Serialize, Deserialize, Clone, Type)]
pub struct NexusUser {
    pub uuid: String,
    pub username: String,

    // The access token and refresh token are for the microsoft account and not the minecraft account
    pub access_token: String,
    pub refresh_token: String,

    // access_token expire date
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

    pub async fn get_minecraft_access_token(&mut self) -> String {
        self.refresh_token().await;
        add_user(self.clone());
        match exchange_microsoft_token(self.access_token.as_str()).await {
            Ok(token) => {
                token.access_token().clone().into_inner()
            }
            Err(e) => {
                println!("{}", e);
                panic!("{}", e);
            }
        }
    }

    pub async fn refresh_token(&mut self) {
        let client = get_auth_client("9c203c7d-1816-4d24-87f2-9731ce05e187");

        let (at, rt) = refresh_auth_token(&client, self.refresh_token.clone()).await;

        self.access_token = at;
        self.refresh_token  = rt;
    }
}
