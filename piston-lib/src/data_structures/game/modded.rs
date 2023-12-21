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

/// A data variable entry that depends on the side of the installation
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[derive(Serialize, Deserialize, Debug)]
pub struct SidedDataEntry {
    /// The value on the client
    pub client: String,
    /// The value on the server
    pub server: String,
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    /// (Forge-only)
    pub data: Option<HashMap<String, SidedDataEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// (Forge-only) The list of processors to run after downloading the files
    pub processors: Option<Vec<Processor>>,
}

/// A processor to be ran after downloading the files
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Processor {
    /// Maven coordinates for the JAR library of this processor.
    pub jar: String,
    /// Maven coordinates for all the libraries that must be included in classpath when running this processor.
    pub classpath: Vec<String>,
    /// Arguments for this processor.
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Represents a map of outputs. Keys and values can be data values
    pub outputs: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Which sides this processor shall be ran on.
    /// Valid values: client, server, extract
    pub sides: Option<Vec<String>>,
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A manifest containing information about a mod loader's versions
pub struct Manifest {
    /// The game versions the mod loader supports
    pub game_versions: Vec<Version>,
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[derive(Serialize, Deserialize, Debug, Clone)]
///  A game version of Minecraft
pub struct Version {
    /// The minecraft version ID
    pub id: String,
    /// Whether the release is stable or not
    pub stable: bool,
    /// A map that contains loader versions for the game version
    pub loaders: Vec<LoaderVersion>,
}

#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
#[derive(Serialize, Deserialize, Debug, Clone)]
/// A version of a Minecraft mod loader
pub struct LoaderVersion {
    /// The version ID of the loader
    pub id: String,
    /// The URL of the version's manifest
    pub url: String,
    /// Whether the loader is stable or not
    pub stable: bool,
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