/*

To install forge/neoforge.

1. Download the installer jar from the database
2. Extract the installer jar
3. Download the libraries specified in the install_profile.json
4. Execute the processors
5. Merge the version.json with the client.json

*/

const FORGE_MAVEN_URL: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge";
const NEOFORGE_MAVEN_URL: &str = "https://maven.neoforged.net/releases/net/neoforged/forge";
const NEOFORGE_BETA_MAVEN_URL: &str = "https://maven.neoforged.net/releases/net/neoforged/neoforge";

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::os::windows::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, ExitStatus};
use std::str::FromStr;
use tokio::fs;
use zip::ZipArchive;
use crate::data_structures::game::metadata::piston_version_manifest::PistonMetadata;
use crate::data_structures::game::modded::{ForgeLibrary, InstallProfile, Modloader, PartialVersionInfo, Processor};
use crate::data_structures::game::version::{Argument, ArgumentType, game_version, Library, LibraryDownload, LibraryDownloads, VersionType};
use crate::processes::fs::{extract_zip};
use crate::processes::launcher::args::get_path_from_artifact;
use crate::processes::launcher::installation::{get_version_jar_path, vanilla};
use crate::processes::launcher::installation::modded::merge_partial_version;
use crate::processes::launcher::installation::vanilla::{download_client_jar, get_version_info};
use crate::processes::network::download_from_uri;


// Execute forge processors and create install json
pub async fn do_forge_work(version_metadata: &PistonMetadata, game_id: &str, modloader: &Modloader, loader_id: &str, data_dir: &PathBuf) {
    let bin_dir = &data_dir.join("bin");
    let lib_dir = &data_dir.join("libraries");
    let versions_dir = &data_dir.join("versions");
    let id = format!("{}-loader-{}-{}", modloader, loader_id, game_id);

    fs::remove_dir(bin_dir).await.unwrap_or(());
    fs::create_dir_all(bin_dir).await.unwrap();

    let maven_url = match modloader {
        Modloader::Forge => {
            FORGE_MAVEN_URL
        },
        Modloader::NeoForge => {
            NEOFORGE_MAVEN_URL
        },
        _ => {return},
    };

    // Create installation information
    create_forge_install_info(maven_url, game_id, loader_id, bin_dir).await;

    // Get installation information
    let info = get_install_profile(bin_dir).await;

    // Download libraries needed for the installation
    download_installer_libaries(lib_dir, info.libraries.clone()).await.unwrap();

    // Download Vanilla Jar for processors
    vanilla::download_client_json(&version_metadata, &game_id, &versions_dir).await.unwrap();
    let v_info = vanilla::get_version_info(&game_id, &versions_dir).await.unwrap();
    download_client_jar(&v_info, &versions_dir).await.unwrap();

    let client_jar_path = get_version_jar_path(data_dir, &game_id);

    // Execute the processors
    execute_processors(&info, lib_dir, bin_dir, client_jar_path).await;


    // Create the version.json
    // You don't have to merge?
    let mut file = File::open(bin_dir.join("version.json")).expect("Unable to open version json");
    let mut forge_version_txt = String::new();
    file.read_to_string(&mut forge_version_txt).expect("Unable to read version json");

    let mut partial_version_info: PartialVersionInfo = serde_json::from_str(&forge_version_txt).expect("Unable to parse partial version json");

    partial_version_info.id = id.clone();

    let new_version_json = merge_partial_version(partial_version_info, v_info);
    fs::create_dir_all(versions_dir.join(&id)).await.unwrap();
    //fs::write(versions_dir.join(&id).join(format!("{}.json", &id)), partial_version_info).await.unwrap();
    fs::write(versions_dir.join(&id).join(format!("{}.json", &id)), serde_json::to_string(&new_version_json).unwrap()).await.unwrap();
}


/// Download the forge installation info from the maven
async fn create_forge_install_info(maven_url: &str, game_id: &str, loader_id: &str, bin_dir: &PathBuf) {
    let jar_link = format!("{}/{}-{}/{}-{}-{}-installer.jar", maven_url, game_id, loader_id, "forge", game_id, loader_id);
    println!("{}", jar_link);
    fs::create_dir_all(bin_dir).await.unwrap();
    let path = bin_dir.join("installer.jar");

    fs::create_dir_all(bin_dir).await.unwrap();
    let bytes = reqwest::get(&jar_link).await.unwrap().bytes().await.unwrap();

    let jar_file = File::create(&path);
    jar_file.unwrap().write_all(&bytes).unwrap();

    extract_zip(path, bin_dir);
}

/// Read the install_profile.json
async fn get_install_profile(bin_dir: &PathBuf) -> InstallProfile {
    let install = match File::open(&bin_dir.join("install_profile.json")) {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to open file: {}", e);
        }
    };

    let mut reader = std::io::BufReader::new(install);

    let info: Result<InstallProfile, _> = serde_json::from_reader(&mut reader);

    match info {
        Ok(info) => {
            println!("{:?}", &info);
            return info;
        },
        Err(e) => panic!("Failed to deserialize JSON: {}", e),
    }
}

/// Download the libraries
async fn download_installer_libaries(lib_dir: &PathBuf, libs: Vec<ForgeLibrary>) -> Result<(), Box<dyn std::error::Error>> {
    for lib in libs {
        if lib.downloads.artifact.url != "" {
            download_from_uri(lib.downloads.artifact.url.as_str(), &lib_dir.join(lib.downloads.artifact.path), Some(lib.downloads.artifact.sha1.as_str()), false).await?;
        }
    }
    Ok(())
}


/// Execute the processors
async fn execute_processors(info: &InstallProfile, lib_dir: &PathBuf, bin_dir: &PathBuf, client_jar_path: PathBuf) {
    println!("Got install profile");
    println!("Starting processors");

    let mut order_num = 0;

    for processor in info.processors.clone().into_iter() {
        order_num = order_num + 1;

        println!("Checking processor: {}", order_num);

        if let Some(ref x) = processor.sides {
            if x.contains(&"server".to_string()) {
                continue
            }
        }
        println!("===============================================================================");

        let mut command = Command::new("java");

        let cp = generate_cp(&processor, info.libraries.clone(), lib_dir).join(if env::consts::OS == "windows" {";"} else {":"}).replace("\\\\","\\");

        let args = generate_processor_args(&processor, &info, lib_dir, bin_dir, &client_jar_path);

        let main_class = match get_main_class_from_jar(lib_dir.join(get_path_from_id(&processor.jar)).to_str().unwrap()) {
            Ok(o) => {o}
            Err(e) => {panic!("Error when finding the main class: {}", e)}
        };

        println!("Main Class: {}", &main_class);

        command.arg("-cp").arg(cp).arg(main_class).args(args);
        command.current_dir(lib_dir);
        let output = command.output().expect("Failed to execute command");
        let output_str = std::str::from_utf8(&output.stdout).unwrap_or("None");
        for line in output_str.lines() {
            println!("{}", line);
        }

        println!("Err: {}", std::str::from_utf8(&output.stderr).unwrap_or("None"));

        match command.status() {
            Ok(status) => {
                if status == ExitStatus::from_raw(1) {
                    panic!("Oh no the command failed with exit code 1");
                }
                else {
                    println!("Command exited with {}", output.status);
                }
            }
            Err(e) => {
                panic!("Oh no the command failed. {}", e);
            }
        }
    }
}


/// Generate forge install class paths
fn generate_cp(processor: &Processor, libraries: Vec<ForgeLibrary>, lib_dir: &PathBuf) -> Vec<String> {
    libraries.iter().filter(|l| processor.classpath.contains(&l.name) || &processor.jar == &l.name).map(|l| {
        lib_dir.join(&l.downloads.artifact.path).to_string_lossy().to_string()
    }).collect()
}


/// Generate processor arguments
fn generate_processor_args(processor: &Processor, info: &InstallProfile, lib_dir: &PathBuf, bin_dir: &PathBuf, client_jar_path: &PathBuf) -> Vec<String> {
    let x = processor.args.clone().into_iter().map(|mut a| {
        for (key, value) in info.data.iter() {
            let c = a.clone();

            a = a.replace(&format!("{{{}}}", key), &value.client)
                .replace("{SIDE}", "client")
                .replace("{ROOT}", lib_dir.parent().unwrap().to_str().unwrap())
                .replace("{MINECRAFT_JAR}", client_jar_path.to_str().unwrap());
        }
        if a.starts_with("[") && a.ends_with("]")  {
            let id = a.chars().skip(1).take(a.len() - 2).collect::<String>();
            a = lib_dir.join(get_path_from_id(&id)).to_string_lossy().to_string() // Join the parts with \
        } else if a.starts_with("'") && a.ends_with("'") {
            a = a.chars().skip(1).take(a.len() - 2).collect::<String>();
        } else if a.contains("\\") || a.contains("/") {
            println!("Found a path: {}", a);
            println!("Bin Dir: {:?}", bin_dir);
            a = bin_dir.join(&a.trim_start_matches('/')).to_string_lossy().to_string();
            println!("A1: {}", &a);
        }
        println!("A2: {}", &a);
        a
    }).collect();
    println!("{:?}", &x);
    x
}

#[test]
fn test_path_from_id() {
    let id = "net.minecraftforge:ForgeAutoRenamingTool:1.0.5:all";
    println!("Path: {}", get_path_from_id(id));
}

fn get_path_from_id(id: &str) -> String {
    println!("ID: {:?}", id);
    // Example: net.minecraftforge:ForgeAutoRenamingTool:1.0.5:all
    // First Part: company
    // Second Part: Name
    // Third Part: Version number
    // Extras: part of t

    let split_1: Vec<&str> = id.split('@').collect();

    // Extension of file
    let file_extension: &str = split_1.get(1).unwrap_or(&"jar");


    let mut parts: Vec<&str> = split_1.first().expect(&format!("Error when finding path of: {}", id)).split(":").collect();
    let first = parts[0].replace(".", "\\");
    let second = parts[1];
    let version = parts[2];
    let mut name = parts[1..].join("-");

    if !file_extension.is_empty() {
        name = format!("{}.{}", name, file_extension);
    }

    let path = vec![&first, second, version, &name].join("\\");
    path

    /*
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
     */
}

fn get_main_class_from_jar(jar_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Finding main class of: {}", jar_path);

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
