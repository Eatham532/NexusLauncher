use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mcstore {
    pub items: Vec<Game>,
    pub signature: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
}