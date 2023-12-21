use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data_structures::game::metadata::mojang_version_manifest::Type;

#[derive(Deserialize, Serialize, Debug, Clone, specta::Type)]
pub struct PistonMetadata {
    pub versions: Vec<MVersion>,
    pub modloaders: Vec<String>,
}

impl PistonMetadata {
    pub fn new() -> PistonMetadata {
        PistonMetadata {
            versions: Vec::new(),
            modloaders: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, specta::Type)]
pub struct MVersion {
    pub id: String,
    pub game_type:Type,
    pub json_url: String,
    pub json_sha1: String,


    // LoaderName | LoaderVersionName | LoaderVersionInfo
    pub modloaders: HashMap<String, HashMap<String, LoaderVersion>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, specta::Type)]
pub struct LoaderVersion {
    pub id: String,
    pub json_url: String,
    pub json_sha1: Option<String>,
    pub stable: bool,
}
