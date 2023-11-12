use std::{env, fs};
use std::collections::HashMap;
use std::path::PathBuf;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use crate::data_structures::game::mojang_version_manifest::{Version, VersionManifestRoot};
use crate::data_structures::game::version::{DownloadType, game_version, Library, Rule, RuleAction};
use crate::processes::fs::verify_dir;
use crate::processes::launcher::args::check_rules;
use crate::processes::network::{download_from_uri, unzip_from_uri};

pub async fn get_version_manifest() -> Result<VersionManifestRoot, reqwest::Error> {
    let url : &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
    let response : Response = Client::new()
        .get(url)
        .send()
        .await?;


    let root : VersionManifestRoot = response.json().await?;

    Ok(root)
}

pub async fn download_client_json(version_id: &str, versions_dir: &PathBuf) -> Result<(), String> {
    let version_list = get_version_manifest().await.unwrap().versions;
    let version : Option<&Version> = version_list.iter().find(|&x| {
        version_id == &x.id
    });

    match version {
        Some(v) => {
            println!("Found version: {:?}", v);
        },
        None => {
            println!("No version found with id: {}", version_id);
            return Err(format!("No version found with id: {}", version_id));
        },
    }

    let version_json_dir = versions_dir.join(version_id).join(format!("{}.json", version_id));

    println!("Downloading client.json");
    let _ = download_from_uri(&version.unwrap().url, &version_json_dir, Some(&version.unwrap().sha1), false).await;
    Ok(())
}

pub async fn get_version_info(version_id: &str, versions_dir: &PathBuf) -> Result<game_version, ()> {
    let version_json_path = versions_dir.join(version_id).join(format!("{}.json", version_id));

    if version_json_path.exists() {
        // Read the file
        let contents : String = fs::read_to_string(version_json_path).unwrap();
        // Deserialize the file
        let result : game_version = serde_json::from_str(&contents).unwrap();
        return Ok(result);
    }
    else {
        println!("Err! version.json does not exist!");
        println!("Path: {:?}", version_json_path);
        return Err(());
    }
}

pub async fn download_client_jar(version: &game_version, path: &PathBuf) {
    println!("Downloading client.jar");
    for download in &version.downloads {
        if download.0 == &DownloadType::Client {
            let _ = download_from_uri(&download.1.url, path, Some(&download.1.sha1), false).await;
        }
    }
}

pub async fn download_libraries(lib_dir: &PathBuf, natives_dir: &PathBuf, libraries: Vec<Library>) {
    println!("Downloading libraries...");
    verify_dir(&natives_dir).await;
    verify_dir(&lib_dir).await;
    for library in libraries {
        // Check if the rule allows the library to be downloaded
        if library.rules.is_some() {
            if check_rules(&library.rules.unwrap()) == false {
                continue;
            }
        }


        for download in library.downloads {
            if let Some(artifact) = download.artifact {
                download_from_uri(&artifact.url, &lib_dir.join(&artifact.path.unwrap()), Some(&artifact.sha1), false).await.expect("TODO: panic message");
            }
            else {
                if let Some(url) = &library.url {
                    download_from_uri(url, &lib_dir.join(&library.name), None, false).await.expect("TODO: panic message");
                }
            }


            if download.classifiers.is_some() {
                let current_native = get_current_native();

                for classifier in download.classifiers {
                    for x in classifier {
                        if x.0 == current_native {
                            download_from_uri(&x.1.url, &lib_dir.join(&x.1.path.unwrap()), Some(&x.1.sha1), false).await.expect("TODO: panic message. Classifier");

                            match unzip_from_uri(&x.1.url, natives_dir, Some(&x.1.sha1), false).await
                            {
                                Ok(()) => {},
                                Err(e) => eprintln!("Error unzipping: {}", e),
                            }
                        }
                        else {
                            // println!("Not match {}, {}", x.0, current_native);
                        }
                    }
                }
            }
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct AssetIndex {
    objects: HashMap<String, Object>,
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    hash: String,
    size: i32,
}

pub async fn download_asset_objects(index: AssetIndex, assets_path: &PathBuf) {
    let path = assets_path.join("objects");
    for asset in index.objects {
        let download_path = path.join(&asset.1.hash[..2]).join(&asset.1.hash);
        match download_from_uri(&*format!("https://resources.download.minecraft.net/{}/{}", &asset.1.hash[..2], &asset.1.hash), &download_path, Some(&asset.1.hash), false).await {
            Ok(result) => {
                println!("Downloaded asset successfully to {:?}", download_path);
            }
            Err(e) => {
                eprintln!("ERROR! {}", e)
            }
        };
    }
}

pub fn get_current_native() -> String {
    let x = match env::consts::OS {
        "windows" => "natives-windows",
        "macos" => "natives-osx",
        "ios" => panic!("OS not supported"),
        "android" => panic!("OS not supported"),
        &_ => "natives-linux",
    };

    println!("Got current native: {}, {}", x, env::consts::OS);
    x.to_string()
}
