use std::collections::HashMap;
use std::fmt::format;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::task;
use crate::data_structures::game::metadata::piston_version_manifest::PistonMetadata;
use crate::data_structures::game::modded::{Modloader, PartialVersionInfo};
use crate::data_structures::game::version::{Argument, ArgumentType, game_version, Library, LibraryDownload, LibraryDownloads};
use crate::processes::launcher::args::get_path_from_artifact;

pub async fn download_modded_json(metadata: &PistonMetadata, version_id: &String, modloader: &Modloader, loader_id: &   String, versions_dir: &PathBuf) {
    let version_info = metadata.versions.iter().find(|p| p.id.as_str() == version_id).unwrap();

    let mc_json_info = reqwest::get(&version_info.json_url).await.unwrap().json::<game_version>().await.unwrap();
    let loader_json_info = reqwest::get(&version_info.modloaders.get(&modloader.to_string()).unwrap().get(&loader_id.to_string()).unwrap().json_url).await.unwrap().json::<PartialVersionInfo>().await.unwrap();

    let new_info = task::spawn_blocking(|| {
        merge_partial_version(loader_json_info, mc_json_info)
    }).await.unwrap();
    println!("Finished info");

    let name = format!("{}-loader-{}-{}", modloader, loader_id, version_id);
    tokio::fs::create_dir_all(versions_dir.join(&name)).await.unwrap();
    tokio::fs::write(versions_dir.join(&name).join(format!("{}.json", name)), serde_json::to_string(&new_info).unwrap()).await.unwrap();
}

/// Merges a partial version into a complete one
pub fn merge_partial_version(
    partial: PartialVersionInfo,
    merge: game_version,
) -> game_version {
    let merge_id = merge.id.clone();

    game_version {
        arguments: if let Some(partial_args) = partial.arguments {
            if let Some(merge_args) = merge.arguments {
                let mut new_map = HashMap::new();

                fn add_keys(
                    new_map: &mut HashMap<ArgumentType, Vec<Argument>>,
                    args: HashMap<ArgumentType, Vec<Argument>>,
                ) {
                    for (type_, arguments) in args {
                        for arg in arguments {
                            if let Some(vec) = new_map.get_mut(&type_) {
                                vec.push(arg);
                            } else {
                                new_map.insert(type_, vec![arg]);
                            }
                        }
                    }
                }

                add_keys(&mut new_map, merge_args);
                add_keys(&mut new_map, partial_args);

                Some(new_map)
            } else {
                Some(partial_args)
            }
        } else {
            merge.arguments
        },
        asset_index: merge.asset_index,
        assets: merge.assets,
        downloads: merge.downloads,
        id: partial.id,
        java_version: merge.java_version,
        libraries: partial
            .libraries
            .into_iter()
            .chain(merge.libraries)
            .map(|x| {
                println!("{}", &x.name);

                match &x.url {
                    Some(l) => {
                        let sha_path = x.url.clone().unwrap_or("NONE".to_string()).clone() + get_path_from_artifact(x.name.clone().as_str()).unwrap().as_str() + ".sha1";
                        let sha_path = sha_path.as_str();

                        Library {
                            downloads: Some(x.downloads.clone().unwrap_or(LibraryDownloads {
                                artifact: Some(LibraryDownload {
                                    path: Some(get_path_from_artifact(x.name.clone().as_str()).unwrap()),
                                    url: x.url.clone().unwrap().clone() + &*get_path_from_artifact(&x.name.clone().as_str()).unwrap(),
                                    sha1: reqwest::blocking::get(sha_path.replace("http://", "https://")).unwrap().text().unwrap(),
                                    size: 0,
                                }),
                                classifiers: None,

                            })),

                            extract: x.extract.clone(),
                            name: x.name.clone().to_string(),
                            url: x.url.clone(),
                            natives: x.natives.clone(),
                            rules: x.rules.clone(),
                            checksums: x.checksums.clone(),
                        }
                    },
                    _ => {
                        if x.downloads.is_some() {
                            x
                        }
                        else {
                            let url = "https://libraries.minecraft.net/";
                            let path = get_path_from_artifact(x.name.clone().as_str()).unwrap();
                            let sha_path = url.to_owned() + path.clone().as_str() + ".sha1";
                            let sha1 = reqwest::blocking::get(sha_path).unwrap().text().unwrap();

                            Library {
                                downloads: Some(x.downloads.clone().unwrap_or(LibraryDownloads {
                                    artifact: Some(LibraryDownload {
                                        path: Some(path.clone()),
                                        url: url.to_owned() + &path,
                                        sha1,
                                        size: 0,
                                    }),
                                    classifiers: None,

                                })),

                                extract: x.extract.clone(),
                                name: x.name.clone().to_string(),
                                url: Some(url.to_string()),
                                natives: x.natives.clone(),
                                rules: x.rules.clone(),
                                checksums: x.checksums.clone(),
                            }
                        }
                    }
                }

            })
            .collect::<Vec<_>>(),
        main_class: if let Some(main_class) = partial.main_class {
            main_class
        } else {
            merge.main_class
        },
        minecraft_arguments: partial.minecraft_arguments,
        minimum_launcher_version: merge.minimum_launcher_version,
        release_time: partial.release_time,
        time: partial.time,
        type_: partial.type_,
        data: partial.data,
        processors: partial.processors,
        logging: merge.logging,
    }
}