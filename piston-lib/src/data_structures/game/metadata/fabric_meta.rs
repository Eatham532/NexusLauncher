use serde::{Deserialize, Serialize};

pub type FabricGameMetaVersionInfo = Vec<Root>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub loader: Loader,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loader {
    pub separator: String,
    pub build: i64,
    pub maven: String,
    pub version: String,
    pub stable: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionStruct {
    pub version: String,
}