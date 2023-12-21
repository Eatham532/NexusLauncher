// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct McVersionManifest {
    #[serde(rename = "latest")]
    pub latest: Latest,

    #[serde(rename = "versions")]
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Latest {
    #[serde(rename = "release")]
    pub release: String,

    #[serde(rename = "snapshot")]
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Version {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "type")]
    pub version_type: Type,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "time")]
    pub time: String,

    #[serde(rename = "releaseTime")]
    pub release_time: String,

    #[serde(rename = "sha1")]
    pub sha1: String,

    #[serde(rename = "complianceLevel")]
    pub compliance_level: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type)]
pub enum Type {
    #[serde(rename = "old_alpha")]
    OldAlpha,

    #[serde(rename = "old_beta")]
    OldBeta,

    #[serde(rename = "release")]
    Release,

    #[serde(rename = "snapshot")]
    Snapshot,
}
