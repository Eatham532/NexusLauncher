use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgeVersionsXml {
    #[serde(rename = "groupId")]
    pub group_id: String,

    #[serde(rename = "artifactId")]
    pub artifact_id: String,

    #[serde(rename = "versioning")]
    pub versioning: Versioning,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Versioning {
    #[serde(rename = "latest")]
    pub latest: String,

    #[serde(rename = "release")]
    pub release: String,

    #[serde(rename = "versions")]
    pub versions: Versions,

    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Versions {
    #[serde(rename = "version")]
    pub version: Vec<String>,
}