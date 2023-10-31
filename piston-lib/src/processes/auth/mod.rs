use core::fmt;
use std::error::Error;
use std::fmt::format;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use bytes::Bytes;
use minecraft_msa_auth::MinecraftAuthorizationFlow;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use oauth2::reqwest::{async_http_client, http_client};
use oauth2::{AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl, DeviceAuthorizationUrl, StandardRevocableToken, RequestTokenError, RefreshToken, RefreshTokenRequest};
use reqwest::{Client, Url};
use reqwest::header::{AUTHORIZATION, HeaderValue};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

const DEVICE_CODE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const MSA_AUTHORIZE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const MSA_TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";

pub fn get_auth_client(client_id: &str) -> BasicClient {
    BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new(MSA_AUTHORIZE_URL.to_string()).unwrap(),
        Some(TokenUrl::new(MSA_TOKEN_URL.to_string()).unwrap()),
    )
        .set_device_authorization_url(DeviceAuthorizationUrl::new(DEVICE_CODE_URL.to_string()).unwrap())
}

pub async fn get_login_details(client: &BasicClient) -> Result<StandardDeviceAuthorizationResponse, String> {
    let details = client
        .exchange_device_code().unwrap()
        .add_scope(Scope::new("XboxLive.signin XboxLive.offline_access".to_string()))
        .request_async(async_http_client)
        .await;

    match details {
        Ok(details) => {
            Ok(details)
        }

        Err(e) => {
            Err(e.to_string())
        }
    }
}

pub async fn wait_for_auth_response(client: &BasicClient, device_auth_response: StandardDeviceAuthorizationResponse) -> Result<BasicTokenResponse, String> {
    return match client.exchange_device_access_token(&device_auth_response)
        .request_async(async_http_client, tokio::time::sleep, None).await {

        Ok(resp) => {
            Ok(resp)
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

pub async fn refresh_auth_token(client: &BasicClient, r_token: String) -> (String, String) {
    let refresh_token = RefreshToken::new(r_token);

    let token_response = client.exchange_refresh_token(&refresh_token)
        .add_scope(Scope::new("XboxLive.signin XboxLive.offline_access".to_string()))
        .request_async(async_http_client).await.unwrap();


    let access_token = token_response.access_token().secret().to_string();
    let refresh_token = token_response.refresh_token().unwrap().secret().to_string();

    return (access_token, refresh_token);
}