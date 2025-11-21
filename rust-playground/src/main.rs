pub mod install_tests;

use std::fmt::format;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::fs::{create_dir_all, File};
use std::hash::Hasher;
use piston_lib::data_structures::game::metadata::piston_version_manifest::{LoaderVersion, MVersion, PistonMetadata};
use piston_lib::data_structures::game::modded;
use piston_lib::data_structures::game::version::game_version;
use piston_lib::processes::launcher::args::{format_arguments, get_classpaths};
use piston_lib::processes::launcher::installation::install_from_version_info;
use piston_lib::processes::launcher::installation::modded::merge_partial_version;
use piston_lib::processes::launcher::installation::HandleProgress;
use std::io::{Read, Write};
use std::{env, fs, io};
use zip::ZipArchive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;
use piston_lib::processes::network::download_from_uri;
use std::fs::OpenOptions;
use std::io::prelude::*;
use tokio::task;
use piston_lib::data_structures::game::modded::Modloader;
use piston_lib::processes::launcher::installation::versioning::generate_versions_metadata;



macro_rules! log_and_print {
   ($($arg:tt)*) => {{
       let msg = format!($($arg)*);
       println!("{}", msg);
       let mut file = OpenOptions::new().write(true).append(true).create(true).open("./log.txt").unwrap();
       writeln!(file, "{}", msg).unwrap();
   }}
}


pub struct InstallProgressHandler {

}

impl HandleProgress for InstallProgressHandler {
    fn update_progress(&self, progress: i32, id: &str, message: &str) {
        log_and_print!("{}", message);
    }
}

impl InstallProgressHandler {
    pub fn new() -> Self {InstallProgressHandler {}}
}

const ID: &str = "1.20.4";
const LOADER: &Modloader = &Modloader::Fabric;
const LOADER_ID: &str = "0.14.4";

const FORGE_META_URL: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge";
const GAME_DIR: &str = "C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\tests";
const LIB_DIR: &str = "C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\tests\\libraries";
const BIN_DIR: &str = "C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\tests\\bin";

pub fn get_versions() -> PistonMetadata {
    let metadata_path = PathBuf::from_str("C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\cache\\version_metadata.json").unwrap();
    let data = std::fs::read_to_string(&metadata_path).unwrap();
    serde_json::from_str::<PistonMetadata>(data.as_str()).unwrap()
    /*generate_versions_metadata()*/
}

fn metadata_debugging() {
    let mut fabric_versions: Vec<LoaderVersion> = Vec::new();
    let versions = get_versions();

    for version in versions.versions {
        println!("Version: {}", version.id);
        println!();
        version.modloaders.iter().for_each(|loader| {
            if loader.0 == "fabric" {
                println!("Contains Fabric");
                loader.1.iter().for_each(|fabric_version| {
                    if fabric_versions.iter().find(|v| v.id == fabric_version.1.id).is_none() {
                        fabric_versions.push(fabric_version.1.clone());
                        println!("FOUND A NEW LOADER VERSION: {}", fabric_version.1.id);
                    }
                });
            }
        });
    }
}

#[test]
fn test() {
    let s = "C:\\Users\\eatha\\AppData\\Roaming\\NexusLauncher\\tests\\bin\\com".to_string();
    let replaced = s.replace("\\\\", "\\");
    log_and_print!("{}", replaced);

    let info = get_install_profile();
    log_and_print!("{:?}", &info);
    let bin_dir = PathBuf::from_str(BIN_DIR).unwrap();

    //download_libaries(info.libraries.clone()).await;
}

fn main() {
    install_tests::install();
    install_tests::launch();
}

async fn install_forge() {
    task::spawn_blocking(|| create_install_info()).await.unwrap();

    let info = get_install_profile();
    log_and_print!("{:?}", &info);

    download_libaries(info.libraries.clone()).await;
    task::spawn_blocking(|| processors()).await.unwrap();
}

fn processors() {
    let info = get_install_profile();
    log_and_print!("Got install profile");
    log_and_print!("Starting processors");


    for processor in info.processors.clone().into_iter() {
        if let Some(ref x) = processor.sides {
            if x.contains(&"server".to_string()) {
                continue
            }
        }
        log_and_print!("===============================================================================");

        let mut command = Command::new("java");

        let cp = generate_cp(&processor, info.libraries.clone()).join(if env::consts::OS == "windows" {";"} else {":"}).replace("\\\\","\\");

        let args = generate_args(&processor, &info, GAME_DIR);

        let main_class = match get_main_class_from_jar(format!("{}\\{}.jar", LIB_DIR, get_path_from_id(&processor.jar)).as_str()) {
            Ok(o) => {o}
            Err(e) => {panic!("Error when executing processors: {}", e)}
        };

        log_and_print!("Main Class: {}", &main_class);

        command.arg("-cp").arg(cp).arg(main_class).args(args);
        log_and_print!("Command: {:?}", &command);
        command.current_dir(GAME_DIR);
        let output = command.output().expect("Failed to execute command");
        let output_str = std::str::from_utf8(&output.stdout).unwrap_or("None");
        for line in output_str.lines() {
            log_and_print!("{}", line);
        }

        log_and_print!("Err: {}", std::str::from_utf8(&output.stderr).unwrap_or("None"));
        log_and_print!("Command exited with {}", output.status);
    }
}


fn generate_cp(processor: &Processor, libraries: Vec<Library>) -> Vec<String> {
    let bin_dir = PathBuf::from_str(LIB_DIR).unwrap();

    libraries.iter().filter(|l| processor.classpath.contains(&l.name) || &processor.jar == &l.name).map(|l| {
        bin_dir.join(&l.downloads.artifact.path).to_string_lossy().to_string()
    }).collect()
}

fn generate_args(processor: &Processor, info: &InstallProfile, root: &str) -> Vec<String> {
    let x = processor.args.clone().into_iter().map(|mut a| {
        for (key, value) in info.data.iter() {
            let c = a.clone();

            a = a.replace(&format!("{{{}}}", key), &value.client)
                .replace("{SIDE}", "client")
                .replace("{ROOT}", root)
                .replace("{MINECRAFT_JAR}", "1.20.4.jar");
        }
        if a.starts_with("[") && a.ends_with("]")  {
            let mut id = a.chars().skip(1).take(a.len() - 2).collect::<String>();
            if !id.contains("@") {
                id = id + "@jar";
            }
            a = format!("{}\\{}", LIB_DIR, get_path_from_id(&id)) // Join the parts with \
        } else if a.starts_with("'") && a.ends_with("'") {
            a = a.chars().skip(1).take(a.len() - 2).collect::<String>();
        } else if a.contains("\\") || a.contains("/") {
            log_and_print!("Found a path: {}", a);
            a = format!("{}\\{}", BIN_DIR, a);
        }
        a
    }).collect();
    log_and_print!("{:?}", &x);
    x
}


fn get_path_from_id(id: &String) -> String {
    log_and_print!("ID: {:?}", id);

    let mut parts: Vec<&str> = id.split(':').collect();
    let first_part = parts[0].replace(".", "\\"); // Replace . with \ in the first part
    parts[0] = &first_part; // Update the first part

    let last_part = parts.pop().unwrap(); // Get the last part

    let last_parts: Vec<&str> = last_part.split("@").collect();

    parts.push(last_parts[0]);


    let combined_parts: Vec<&str> = parts[1..].to_vec();

    let filename = if last_parts.len() == 1 {
        format!("{}", combined_parts.join("-"))
    }
    else {
        format!("{}.{}", combined_parts.join("-"), last_parts[1])
    };

    parts.push(filename.as_str());

    parts.join("\\")
}

fn get_main_class_from_jar(jar_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    log_and_print!("Finding main class of: {}", jar_path);

    // Open the jar file
    let mut zip = ZipArchive::new(File::open(jar_path)?)?;

    // Read the META-INF/MANIFEST.MF file
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        if file.name() == "META-INF/MANIFEST.MF" {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            // Parse the main class
            let main_class: Option<&str> = contents.lines().find(|line| line.starts_with("Main-Class:"));
            return match main_class {
                Some(class) => Ok(class[12..].to_string()),
                None => Err(format!("Couldn't find main class in the META-INF/MANIFEST.MF file of {}", jar_path).into()),
            }
        }
    }

    // If we reach here, it means we didn't find the META-INF/MANIFEST.MF file
    Err(format!("Couldn't find META-INF/MANIFEST.MF in {}", jar_path).into())
}



fn get_install_profile() -> InstallProfile {
    let path = format!("{}", crate::BIN_DIR);

    let install = match File::open(format!("{}\\install_profile.json", &path)) {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to open file: {}", e);
        }
    };

    let mut reader = std::io::BufReader::new(install);

    let info: Result<InstallProfile, _> = serde_json::from_reader(&mut reader);

    match info {
        Ok(info) => {
            log_and_print!("{:?}", &info);
            return info;
        },
        Err(e) => panic!("Failed to deserialize JSON: {}", e),
    }
}

fn get_version_json() -> VersionJson {
    let path = format!("{}", crate::BIN_DIR);

    let install = match File::open(format!("{}\\version.json", &path)) {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to open file: {}", e);
        }
    };

    let mut reader = std::io::BufReader::new(install);

    let info: Result<VersionJson, _> = serde_json::from_reader(&mut reader);

    match info {
        Ok(info) => {
            log_and_print!("{:?}", &info);
            return info;
        },
        Err(e) => panic!("Failed to deserialize JSON: {}", e),
    }
}

fn create_install_info() {
    let jar_link = format!("{}/{}/{}-{}-installer.jar", FORGE_META_URL, LOADER_ID, "forge", LOADER_ID);
    log_and_print!("{}", jar_link);
    fs::create_dir_all(BIN_DIR).unwrap();
    let path = format!("{}\\installer.jar", BIN_DIR);

    fs::create_dir_all(BIN_DIR).unwrap();
    let bytes = reqwest::blocking::get(&jar_link).unwrap().bytes().unwrap();
    fs::write(&path, bytes).unwrap();

    let file = File::open(&path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = PathBuf::from(format!("{}\\{}", BIN_DIR, file.name()));

        {
            let comment = file.comment();
            if !comment.is_empty() {
                log_and_print!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            log_and_print!("File {} extracted to \"{}\"", i, outpath.as_path().display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            log_and_print!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

async fn download_libaries(libs: Vec<Library>) {
    let bin_dir = PathBuf::from_str(LIB_DIR).unwrap();

    for lib in libs {
        if lib.downloads.artifact.url != "" {
            download_from_uri(lib.downloads.artifact.url.as_str(), &bin_dir.join(lib.downloads.artifact.path), Some(lib.downloads.artifact.sha1.as_str()), false).await.unwrap();
        }
    }
}


/*

To install forge/neoforge.

1. Download the installer jar from the database
2. Extract the installer jar
3. Download the libraries specified in the install_profile.json
4. Execute the processors
5. Merge the version.json with the client.json

*/


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallProfile {
    #[serde(rename = "_comment")]
    comment: Vec<String>,

    #[serde(rename = "hideExtract")]
    hide_extract: bool,

    #[serde(rename = "spec")]
    spec: i64,

    #[serde(rename = "profile")]
    profile: String,

    #[serde(rename = "version")]
    version: String,

    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "minecraft")]
    minecraft: String,

    #[serde(rename = "serverJarPath")]
    server_jar_path: String,

    #[serde(rename = "data")]
    data: HashMap<String, Datum>,

    #[serde(rename = "processors")]
    processors: Vec<Processor>,

    #[serde(rename = "libraries")]
    libraries: Vec<Library>,

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
    client: String,

    #[serde(rename = "server")]
    server: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Library {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "downloads")]
    downloads: Downloads,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Downloads {
    #[serde(rename = "artifact")]
    artifact: Artifact,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "sha1")]
    sha1: String,

    #[serde(rename = "size")]
    size: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Processor {
    #[serde(rename = "sides")]
    sides: Option<Vec<String>>,

    #[serde(rename = "jar")]
    jar: String,

    #[serde(rename = "classpath")]
    classpath: Vec<String>,

    #[serde(rename = "args")]
    args: Vec<String>,

    #[serde(rename = "outputs")]
    outputs: Option<Outputs>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Outputs {
    #[serde(rename = "{MC_UNPACKED}")]
    mc_unpacked: Option<String>,

    #[serde(rename = "{MAPPINGS}")]
    mappings: Option<String>,

    #[serde(rename = "{MOJMAPS}")]
    mojmaps: Option<String>,

    #[serde(rename = "{MERGED_MAPPINGS}")]
    merged_mappings: Option<String>,

    #[serde(rename = "{MC_SRG}")]
    mc_srg: Option<String>,

    #[serde(rename = "{PATCHED}")]
    patched: Option<String>,
}






// Version.json


#[derive(Clone, Debug, Serialize, Deserialize)]
struct Arguments {
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct VersionJson {
    pub _comment: Vec<String>,
    pub id: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    #[serde(rename = "inheritsFrom")]
    pub inherits_from: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub libraries: Vec<Library>,
    pub arguments: Arguments,
}