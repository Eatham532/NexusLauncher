mod structs;

use std::path::PathBuf;
use minecraft_msa_auth::{MinecraftAuthenticationResponse, MinecraftAuthorizationError, MinecraftAuthorizationFlow};
use oauth2::http::StatusCode;
use reqwest::Client;
use reqwest::header::AUTHORIZATION;
pub use structs::*;

const MOJANG_API_URL: &str = "https://api.minecraftservices.com";

pub async fn exchange_microsoft_token(microsoft_token: &str) -> Result<MinecraftAuthenticationResponse, MinecraftAuthorizationError> {
    let mc_flow = MinecraftAuthorizationFlow::new(Client::new());

    let mc_token = mc_flow.exchange_microsoft_token(microsoft_token).await;
    println!("Returning mojang token");
    mc_token
}

fn get_api_path(endpoint: &str) -> String {
    format!("{}{}", MOJANG_API_URL, endpoint)
}



// This is the part where you call the mojang api




/// This endpoint allows an authenticated user to check if they own a copy of Minecraft.
pub async fn verify_game_ownership(bearer_token: &str) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let res = match client.get(get_api_path("/entitlements/mcstore"))
        .header(AUTHORIZATION, "Bearer ".to_owned() + bearer_token)
        .send()
        .await {
        Ok(res) => res,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    return match res.status().as_str() {
        "200" => {
            let json = res.json::<entitlements::mc_store::Mcstore>().await.unwrap();

            return if json.items.is_empty() {
                Ok(false)
            }
            else {
                Ok(true)
            }
        }
        "400" => {
            Err("Bad request. Required JWT [user] not specified".to_string())
        }
        _ => {
            Err("Invalid Status code".to_string())
        }
    }
}

/// This endpoint allows an authenticated user to view their user information (birthday, userId value, email, if it has security questions, etc).
pub async fn get_user_info(bearer_token: &str) -> Result<user::User, String> {
    let client = reqwest::Client::new();
    let res = match client.get(get_api_path("/user"))
        .header(AUTHORIZATION, "Bearer ".to_owned() + bearer_token)
        .send()
        .await {
        Ok(res) => res,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    return match res.status().as_str() {
        "200" => {
            Ok(res.json::<user::User>().await.unwrap())
        }
        "401" => {
            Err("Unauthorized. User not authenticated".to_string())
        }
        _ => {
            Err("Invalid Status code".to_string())
        }
    }
}

/// This endpoint allows for an authenticated user to view information about their Minecraft profile.
pub async fn get_profile_info(bearer_token: &str) -> Result<minecraft::profile::Profile, String> {
    println!("Grabbing profile info");
    let client = reqwest::Client::new();
    let res = match client.get(get_api_path("/minecraft/profile"))
        .header(AUTHORIZATION, "Bearer ".to_owned() + bearer_token)
        .send()
        .await {
        Ok(res) => res,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    println!("Matching info");
    // Error if user does not have a minecraft account
    return match res.status().as_str() {
        "200" => {
            Ok(res.json::<minecraft::profile::Profile>().await.unwrap())
        }
        "401" => {
            Err("Unauthorized".to_string())
        }
        "404" => {
            Err("User has not bought the game".to_string())
        }
        _ => {
            Err("Invalid Status code".to_string())
        }
    }
}