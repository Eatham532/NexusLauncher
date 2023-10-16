use std::{env, fs};
use std::path::PathBuf;
use reqwest::{Client, Response};
use crate::data_structures::game::mojang_version_manifest::{Version, VersionManifestRoot};
use crate::data_structures::game::version::{DownloadType, game_version, Library};
use crate::processes::fs::verify_dir;
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

    // Read the file
    let contents : String = fs::read_to_string(version_json_path).unwrap();
    // Deserialize the file
    let result : game_version = serde_json::from_str(&contents).unwrap();

    Ok(result)
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
        for download in library.downloads {
            for file in download.artifact {
                download_from_uri(&file.url, &lib_dir.join(&file.path.unwrap()), Some(&file.sha1), false).await.expect("TODO: panic message");
            }

            if Some(&download.classifiers).is_some() {
                let current_native = get_current_native();

                for classifier in download.classifiers {
                    for x in classifier {
                        if x.0 == current_native {
                            /*println!("Downloading uri...");
                            download_from_uri(&x.1.url, &lib_dir.join(&x.1.path.unwrap()), Some(&x.1.sha1), false).await.expect("TODO: panic message. Classifier");
*/
                            match unzip_from_uri(&x.1.url, natives_dir, Some(&x.1.sha1), false).await
                            {
                                Ok(()) => println!("Unzip successful"),
                                Err(e) => eprintln!("Error: {}", e),
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

pub fn get_current_native() -> String {
    let x = match env::consts::OS {
        "windows" => "natives-windows",
        "macos" => "natives-osx",
        "ios" => panic!("OS not supported"),
        "android" => panic!("OS not supported"),
        &_ => "natives-linux",
    };
    x.to_string()
}
