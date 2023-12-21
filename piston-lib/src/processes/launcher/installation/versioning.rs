use std::collections::HashMap;
use std::path::Path;
use rayon::prelude::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use crate::data_structures::game::metadata::fabric_meta::{FabricGameMetaVersionInfo, VersionStruct};
use crate::data_structures::game::metadata::forge_meta::ForgeVersionsXml;
use crate::data_structures::game::metadata::mojang_version_manifest::McVersionManifest;
use crate::data_structures::game::metadata::piston_version_manifest::{LoaderVersion, MVersion, PistonMetadata};

const VANILLA_META_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";
const FORGE_LATEST_URL: &str = "https://files.minecraftforge.net/maven/net/minecraftforge/forge/promotions_slim.json";
// const FORGE_META_URL: &str = "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json";
const FORGE_META_URL: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge";
const NEOFORGE_META_URL: &str = "https://maven.neoforged.net/releases/net/neoforged/forge";
const NEOFORGE_BETA_META_URL: &str = "https://maven.neoforged.net/releases/net/neoforged/neoforge";
const FABRIC_META_URL: &str = "https://meta.fabricmc.net/v2/versions";
const QUILT_META_URL: &str = "https://meta.quiltmc.org/v3/versions";

pub fn generate_versions_metadata() -> PistonMetadata {

    let mut version_meta: PistonMetadata = PistonMetadata::new();

    println!("Vanilla");
    // Vanilla
    let vanilla_meta: McVersionManifest = reqwest::blocking::get(VANILLA_META_URL).unwrap().json::<McVersionManifest>().unwrap();
    let mut vanilla_versions: HashMap<String, LoaderVersion> = HashMap::new();

    for version in &vanilla_meta.versions {
        version_meta.versions.push(MVersion {
            id: version.id.clone(),
            game_type: version.version_type.clone(),
            json_url: version.url.clone(),
            json_sha1: version.sha1.clone(),

            modloaders: HashMap::new(),
        });
    }
    version_meta.modloaders.push("vanilla".to_string());

    // Fabric
    println!("Fabric");
    version_meta = add_fabric_modloader("Fabric", FABRIC_META_URL, version_meta);

    // Quilt
    println!("Quilt");
    version_meta = add_fabric_modloader("Quilt", QUILT_META_URL, version_meta);

    println!("Forge");
    // Forge
    version_meta = add_forge_modloader("Forge", FORGE_META_URL, version_meta);

    println!("NeoForge");
    // NeoForge
    version_meta = add_forge_modloader("NeoForge", NEOFORGE_META_URL, version_meta);

    // NeoForge Beta (Currently unknown how to detect which version the beta supports so disabled)
    // version_meta = add_forge_modloader("NeoForge", NEOFORGE_BETA_META_URL, version_meta);

    version_meta
}

fn add_fabric_modloader(loader_name: &str, repo: &str, mut version_meta: PistonMetadata) -> PistonMetadata {
    let loader_version_list: Vec<String> = match reqwest::blocking::get(format!("{}/game", repo)) {
        Ok(result) => result.json::<Vec<VersionStruct>>().unwrap().iter().map(|v| v.version.clone()).collect(),
        Err(e) => {
            eprintln!("Error when requesting for {} versions. \n {}", loader_name, e);
            return version_meta;
        }
    };

    version_meta.modloaders.push(loader_name.to_lowercase().to_string());

    version_meta.versions.par_iter_mut().filter(|v| loader_version_list.contains(&v.id))
        .for_each(|version| {
            /*if !loader_version_list.contains(&version.id) { continue };*/
            println!("{}", version.id);
            match reqwest::blocking::get(format!("{}/loader/{}", repo, version.id)) {
                Ok(r) => {
                    if r.status().is_success() {
                        let loader_versions = r.json::<FabricGameMetaVersionInfo>().unwrap();
                        let mut loader_info : HashMap<String, LoaderVersion> = HashMap::new();

                        for l_version in loader_versions {
                            loader_info.insert(l_version.loader.version.clone(), LoaderVersion {
                                id: l_version.loader.version.clone(),
                                json_url: format!("{}/loader/{}/{}/profile/json", repo, version.id, l_version.loader.version.clone()),
                                json_sha1: None,
                                stable: l_version.loader.stable.unwrap_or(if l_version.loader.version.clone().contains("beta") {false} else {true}),
                            });
                        }

                        version.modloaders.insert(loader_name.to_lowercase().to_string(), loader_info);
                    }
                    else {
                        println!("Error: {}", r.status());
                        println!("Error most likely due to the version not existing on {}", loader_name);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            };
        });

    version_meta
}

fn add_forge_modloader(loader_name: &str, repo: &str, mut version_meta: PistonMetadata) -> PistonMetadata {
    let xml = match reqwest::blocking::get(format!("{}/maven-metadata.xml", repo)) {
        Ok(result) => result.text().unwrap(),
        Err(e) => {
            eprintln!("Error when requesting for {} versions. \n {}", loader_name, e);
            return version_meta;
        }
    };

    version_meta.modloaders.push(loader_name.to_lowercase().to_string());

    let json: ForgeVersionsXml = match serde_xml_rs::from_str(&xml) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error when parsing request \n {}", e);
            return version_meta;
        }
    };

    let mut version_list : HashMap<String, String> = HashMap::new();


    for version in version_meta.versions.iter_mut() {
        // Currently NeoForge has two repo's with version info. So we must get the other loader info (if there is any)
        let mut loader_info: HashMap<String, LoaderVersion> = version.modloaders.get(&loader_name.to_lowercase().to_string()).unwrap_or(&HashMap::new()).to_owned();

        json.versioning.versions.version.iter().filter(|e| e.starts_with(&version.id))
            .for_each(|v| {
                let split = v.split("-");
                let l_version = split.last().unwrap().to_string();
                loader_info.insert(l_version.clone(),
                                   LoaderVersion {
                                       id: l_version,
                                       json_url: "".to_string(),
                                       json_sha1: None,
                                       stable: false,
                                   });
            });

        if !loader_info.is_empty() { version.modloaders.insert(loader_name.to_lowercase().to_string(), loader_info); }
    }


    version_meta
}