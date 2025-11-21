use std::{env, fs};
use std::collections::HashMap;
use std::path::PathBuf;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use crate::data_structures::game::metadata::mojang_version_manifest::{Version, McVersionManifest};
use crate::data_structures::game::metadata::piston_version_manifest::{MVersion, PistonMetadata};
use crate::data_structures::game::version::{DownloadType, game_version, Library, Rule, RuleAction, LibraryDownloads};
use crate::processes::fs::verify_dir;
use crate::processes::launcher::args::check_rules;
use crate::processes::network::{download_from_uri, DownloadResult, unzip_from_uri};

pub async fn get_version_manifest() -> Result<McVersionManifest, reqwest::Error> {
    let url : &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
    let response : Response = Client::new()
        .get(url)
        .send()
        .await?;


    let root : McVersionManifest = response.json().await?;

    Ok(root)
}

pub async fn download_client_json(metadata: &PistonMetadata, version_id: &str, versions_dir: &PathBuf) -> Result<(), String> {
    let version : Option<&MVersion> = metadata.versions.iter().find(|&x| {
        version_id == &x.id
    });

    match version {
        Some(v) => {

        },
        None => {
            println!("No version found with id: {}", version_id);
            return Err(format!("No version found with id: {}", version_id));
        },
    }

    let version_json_dir = versions_dir.join(version_id).join(format!("{}.json", version_id));

    println!("Downloading client.json");
    let _ = download_from_uri(&version.unwrap().json_url, &version_json_dir, Some(&version.unwrap().json_sha1), false).await;
    Ok(())
}

pub async fn get_version_info(version_id: &str, versions_dir: &PathBuf) -> Result<game_version, ()> {
    let version_json_path = versions_dir.join(version_id).join(format!("{}.json", version_id));

    return if version_json_path.exists() {
        // Read the file
        let contents: String = fs::read_to_string(version_json_path).unwrap();
        // Deserialize the file
        let result: game_version = serde_json::from_str(&contents).unwrap();
        Ok(result)
    } else {
        println!("Err! version.json does not exist!");
        println!("Path: {:?}", version_json_path);
        Err(())
    }
}

pub async fn download_client_jar(version: &game_version, versions_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading client.jar");
    let path = versions_dir.join(&version.id).join(format!("{}.jar", &version.id));

    println!("Path: {:?}", path);
    for download in &version.downloads {
        if download.0 == &DownloadType::Client {
            return match download_from_uri(&download.1.url, &path, Some(&download.1.sha1), false).await {
                Ok(r) => {
                    match r {
                        DownloadResult::Downloaded => {
                            println!("The client jar was downloaded successfully to {}", path.to_string_lossy().to_string());
                            Ok(())
                        }
                        DownloadResult::Exists => {
                            println!("The client jar already exists");
                            Ok(())
                        }
                        DownloadResult::Error => {
                            println!("The was an error downloading the client jar");
                            Err("The was an error downloading the client jar".into())
                        }
                    }
                },
                Err(e) => {
                    println!("The was an error downloading the client jar. {}", e);
                    Err(format!("The was an error downloading the client jar. {}", e).into())
                }
            }
        }
    }
    Err("Could not find client jar".into())
}

pub async fn download_libraries(lib_dir: &PathBuf, natives_dir: &PathBuf, libraries: Vec<Library>) {
    println!("Downloading libraries...");
    verify_dir(&natives_dir).await;
    verify_dir(&lib_dir).await;
    for library in libraries {
        // Check if the rule allows the library to be downloaded
        if let Some(rules) = library.rules {
            if check_rules(&rules) == false {
                continue;
            }
        }


        if let Some(download) = library.downloads {
            if let Some(artifact) = download.artifact {
                if &artifact.url != "" {
                    download_from_uri(&artifact.url.replace("http://", "https://"), &lib_dir.join(&artifact.path.unwrap()), Some(&artifact.sha1), false).await.expect("TODO: panic message");
                }
            }
            else {
                if let Some(url) = &library.url {
                    if &url != &"" {
                        download_from_uri(url.replace("http://", "https://").as_str(), &lib_dir.join(&library.name), None, false).await.expect("TODO: panic message");
                    }
                }
            }


            if let Some(classifier) = download.classifiers {
                for x in classifier {
                    if check_native(x.0) {
                        // download_from_uri(&x.1.url, &lib_dir.join(&x.1.path.unwrap()), Some(&x.1.sha1), false).await.expect("TODO: panic message. Classifier");

                        if &x.1.url != "" {
                            match unzip_from_uri(&x.1.url.replace("http://", "https://"), natives_dir, Some(&x.1.sha1), false).await
                            {
                                Ok(()) => {},
                                Err(e) => eprintln!("Error unzipping: {}", e),
                            }
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
                /*println!("Downloaded asset successfully to {:?}", download_path);*/
            }
            Err(e) => {
                eprintln!("ERROR! {}", e)
            }
        };
    }
}

pub fn check_native(native: String) -> bool {
    native.replace("osx", "macos").contains(env::consts::OS)
}
