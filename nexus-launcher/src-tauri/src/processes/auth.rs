use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use serde::Serialize;
use specta::Type;
use tauri::{State, Window};
use tokio::sync::oneshot;
use piston_lib::processes::auth::{get_auth_client, get_login_details, wait_for_auth_response};
use oauth2::basic::BasicTokenResponse;
use piston_lib::processes::api::mojang::{exchange_microsoft_token, get_profile_info};
use std::time::Instant;
use crate::user::NexusUser;
use crate::add_user;
use std::ops::Add;
use chrono::Utc;
use oauth2::TokenResponse;
use chrono::Duration;

lazy_static! {
    static ref CANCEL_SENDER: Arc<Mutex<Option<oneshot::Sender<()>>>> = Arc::new(Mutex::new(None));
}

#[derive(Clone, Serialize, Type)]
struct AuthPayload {
    stage: AuthStage,
}

#[derive(Clone, Serialize, Type)]
pub enum AuthStage {
    Start,
    AuthCode {
        code: String,
        url: String,
    },
    Cancelled,
    Complete,
}

#[tauri::command]
#[specta::specta]
pub async fn start_login(window: Window) -> Result<(), String> {
    println!("Starting login");

    // Cancel any previous auth processes
    let client = get_auth_client("9c203c7d-1816-4d24-87f2-9731ce05e187").await;


    let login_info = get_login_details(&client).await.unwrap();

    println!("Emitting auth_login");
    window.emit("auth_login", AuthPayload {
        stage: AuthStage::AuthCode {
            code: login_info.user_code().secret().to_string(),
            url: login_info.verification_uri().to_string(),
        },
    }).unwrap();

    cancel_auth().await.unwrap();

    let (cancel_tx, cancel_rx) = oneshot::channel();

    {
        let mut _cancel_sender = CANCEL_SENDER.lock().unwrap();
        *_cancel_sender = Some(cancel_tx);
        // Drops _cancel_sender
    }

    let response: Result<BasicTokenResponse, String> = tokio::select! {
        resp = wait_for_auth_response(&client, login_info) => {
            // Login completed
            resp
        }
        _ = cancel_rx => {
            // Login was cancelled
            window.emit("auth_login", AuthPayload {
                stage: AuthStage::Cancelled,
            }).unwrap();
            Err("Auth was cancelled by user".to_string())
        }
    };

    return match response {
        Ok(r) => {
            println!("Response: {:?}", r);
            println!("Emitting success");

            // Update user.json
            let access_token = &*r.access_token().secret();

            let mc_token = match exchange_microsoft_token(access_token).await {
                Ok(token) => token,
                Err(e) => {
                    println!("{}", e);
                    return Err(e.to_string());
                }
            };
            let bearer = &*mc_token.access_token().clone().into_inner().into_boxed_str();

            let profile_info = match get_profile_info(bearer).await {
                Ok(info) => info,
                Err(e) => {
                    println!("{}", e);
                    return Err(e);
                }
            };

            let expire_date = Utc::now().checked_add_signed(Duration::from_std(r.expires_in().unwrap()).unwrap()).unwrap();

            let user: NexusUser = NexusUser::new(profile_info.id, profile_info.name, access_token.to_string(), r.refresh_token().unwrap().secret().to_string(), expire_date.to_rfc3339());

            add_user(user);

            window.emit("auth_login", AuthPayload {
                stage: AuthStage::Complete,
            }).unwrap();

            Ok(())
        },
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn cancel_auth() -> Result<(), String> {
    let mut cancel_tx = CANCEL_SENDER.lock().unwrap();

    if let Some(tx) = cancel_tx.take() {
        tx.send(()).unwrap();
        *cancel_tx = None;
    }
    Ok(())
}