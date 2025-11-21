use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Capes {
    pub id: String,
    pub state: String,
    pub url: String,
    pub alias: String,
}

#[derive(Serialize, Deserialize)]
pub struct Skins {
    pub id: String,
    pub state: String,
    pub url: String,
    pub variant: String,
    pub alias: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub skins: Vec<Skins>,
    pub capes: Vec<Capes>,
}