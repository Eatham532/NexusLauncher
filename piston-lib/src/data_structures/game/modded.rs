//*  Code in this document was taken from modrinth's daedalus                    *//
//*  https://github.com/modrinth/daedalus/blob/master/daedalus/src/minecraft.rs  *//


use crate::data_structures::game::version::{
    Argument, ArgumentType, Library, game_version, VersionType
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use specta::Type;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};

/// The latest version of the format the fabric model structs deserialize to
pub const CURRENT_FABRIC_FORMAT_VERSION: usize = 0;
/// The latest version of the format the fabric model structs deserialize to
pub const CURRENT_FORGE_FORMAT_VERSION: usize = 0;
/// The latest version of the format the quilt model structs deserialize to
pub const CURRENT_QUILT_FORMAT_VERSION: usize = 0;
/// The latest version of the format the neoforge model structs deserialize to
pub const CURRENT_NEOFORGE_FORMAT_VERSION: usize = 0;

/// The dummy replace string library names, inheritsFrom, and version names should be replaced with
pub const DUMMY_REPLACE_STRING: &str = "${modrinth.gameVersion}";


#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A partial version returned by fabric meta
pub struct PartialVersionInfo {
    /// The version ID of the version
    pub id: String,
    /// The version ID this partial version inherits from
    pub inherits_from: String,
    /// The time that the version was released
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub release_time: DateTime<Utc>,
    /// The latest time a file in this version was updated
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The classpath to the main class to launch the game
    pub main_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// (Legacy) Arguments passed to the game
    pub minecraft_arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arguments passed to the game or JVM
    pub arguments: Option<HashMap<ArgumentType, Vec<Argument>>>,
    /// Libraries that the version depends on
    pub libraries: Vec<Library>,
    #[serde(rename = "type")]
    /// The type of version
    pub type_: VersionType,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Eq, PartialEq, Hash)]
pub enum Modloader {
    Vanilla,
    Fabric,
    Quilt,
    Forge,
    NeoForge
}

impl fmt::Display for Modloader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modloader::Vanilla => write!(f, "vanilla"),
            Modloader::Fabric => write!(f, "fabric"),
            Modloader::Quilt => write!(f, "quilt"),
            Modloader::Forge => write!(f, "forge"),
            Modloader::NeoForge => write!(f, "neoforge"),
        }
    }
}



// Forge Install Profile

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallProfile {
    #[serde(rename = "spec")]
    spec: i64,

    #[serde(rename = "profile")]
    profile: String,

    #[serde(rename = "version")]
    version: String,

    #[serde(rename = "path")]
    path: Option<String>,

    #[serde(rename = "minecraft")]
    minecraft: String,

    #[serde(rename = "serverJarPath")]
    server_jar_path: String,

    #[serde(rename = "data")]
    pub(crate) data: HashMap<String, Datum>,

    #[serde(rename = "processors")]
    pub processors: Vec<Processor>,

    #[serde(rename = "libraries")]
    pub libraries: Vec<ForgeLibrary>,

    #[serde(rename = "icon")]
    icon: String,

    #[serde(rename = "json")]
    json: String,

    #[serde(rename = "logo")]
    logo: String,

    #[serde(rename = "mirrorList")]
    mirror_list: String,

    #[serde(rename = "welcome")]
    welcome: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "client")]
    pub(crate) client: String,

    #[serde(rename = "server")]
    server: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForgeLibrary {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "downloads")]
    pub(crate) downloads: Downloads,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Downloads {
    #[serde(rename = "artifact")]
    pub(crate) artifact: Artifact,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(rename = "path")]
    pub(crate) path: String,

    #[serde(rename = "url")]
    pub(crate) url: String,

    #[serde(rename = "sha1")]
    pub(crate) sha1: String,

    #[serde(rename = "size")]
    size: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Processor {
    #[serde(rename = "sides")]
    pub sides: Option<Vec<String>>,

    #[serde(rename = "jar")]
    pub jar: String,

    #[serde(rename = "classpath")]
    pub classpath: Vec<String>,

    #[serde(rename = "args")]
    pub args: Vec<String>,

    #[serde(rename = "outputs")]
    pub outputs: Option<HashMap<String, String>>,
}