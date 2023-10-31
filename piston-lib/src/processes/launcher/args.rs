use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, format};
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::data_structures::game::version::Argument::{Normal, Ruled};
use dunce::canonicalize;
use serde::{Deserialize, Serialize};
use crate::data_structures::game::version::{Argument, ArgumentType, ArgumentValue, Library, Rule, RuleAction, VersionType};
use crate::data_structures::game::version::ArgumentValue::{Many, Single};

/// TODO: Legacy arguments are different



/// Game window size
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WindowSize(pub u16, pub u16);

impl Default for WindowSize {
    fn default() -> Self {
        Self(854, 480)
    }
}

pub struct MinecraftArgs {
    pub access_token: String,
    pub username: String,
    pub uuid: String,
    pub version: String,
    pub asset_index_name: String,
    pub game_directory: PathBuf,
    pub assets_directory: PathBuf,
    pub version_type: VersionType,
    pub resolution: WindowSize,
}

pub struct  JvmArgs {
    pub natives_path: PathBuf,
    pub libraries_path: PathBuf,
    pub class_paths: String,
    pub version_name: String,
    pub java_arch: String,
}




pub fn format_arguments(arguments: HashMap<ArgumentType, Vec<Argument>>, mc_args: MinecraftArgs, jvm_args: &JvmArgs, main_class: String) -> Vec<String> {
    // Make sure jvm args are before mc_args

    let mut mc_args_vec: Vec<String> = Vec::new();
    let mut jvm_args_vec: Vec<String> = Vec::new();
    for args in arguments {
        match args.0 {
            ArgumentType::Game => {
                for arg in args.1 {
                    match arg {
                        Normal(a) => {
                            //println!("Normal arg: {}", parse_minecraft_arg(&a.clone(), &mc_args));
                            mc_args_vec.push(parse_minecraft_arg(&a, &mc_args));
                        },
                        _ => {

                        }
                    }
                }
            },
            ArgumentType::Jvm => {
                for arg in args.1 {
                    match arg.clone() {
                        Normal(a) => {
                            println!("{}", parse_jvm_argument(a.clone(), &jvm_args));
                            jvm_args_vec.push(parse_jvm_argument(a, &jvm_args));
                        },
                        Ruled { rules, value } => {
                            /// TODO: Clean up this code

                            if check_rules(rules) == true {
                                match value {
                                    Single(v) => {
                                        //println!("{}", parse_jvm_argument(v.clone(), &jvm_args));
                                        jvm_args_vec.push(parse_jvm_argument(v, &jvm_args));
                                    },
                                    Many(v) => {
                                        for arg in v {
                                            //println!("{}", parse_jvm_argument(arg.clone(), &jvm_args));
                                            jvm_args_vec.push(parse_jvm_argument(arg, &jvm_args));
                                        }
                                    }
                                }
                            };
                        }
                        _ => {}
                    }
                }
            },
        }
    }
/*    jvm_args_vec.push("-Dhttp.proxyHost=10.101.174.105 -Dhttp.proxyPort=9666".to_string());*/

    jvm_args_vec.push("-Dorg.lwjgl.util.Debug=true".to_string());
    jvm_args_vec.push(main_class);
    jvm_args_vec.append(&mut mc_args_vec);
    jvm_args_vec
}

pub fn format_arguments_legacy(arguments: String, mc_args: MinecraftArgs, jvm_args: &JvmArgs, main_class: String) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    // JVM args
    args.push(format!(
        "-Djava.library.path={}",
        &*wrap(&canonicalize(&jvm_args.natives_path).unwrap()
            .to_string_lossy().to_string())
    ));
    args.push("-cp".to_string());
    args.push(jvm_args.class_paths.clone());

    args.push("-Dorg.lwjgl.util.Debug=true".to_string());
    // Minecraft args
    args.push(main_class);
    for arg in arguments.split(" ") {
        args.push(parse_minecraft_arg(arg, &mc_args));
    }
    args
}

fn parse_minecraft_arg(argument: &str,
                       mc_args: &MinecraftArgs,
) -> String {
    println!("Start");
    argument.replace("${accessToken}", &mc_args.access_token)
        .replace("${auth_access_token}", &mc_args.access_token)
        .replace("${auth_session}", &mc_args.access_token)
        .replace("${auth_player_name}", &*wrap(&mc_args.username))
        // TODO: add auth xuid eventually
        .replace("${auth_xuid}", "0")
        .replace("${auth_uuid}", &mc_args.uuid)
        .replace("${uuid}", &mc_args.uuid)
        .replace("${clientid}", "c4502edb-87c6-40cb-b595-64a280cf8906")
        .replace("${user_properties}", "{}")
        .replace("${user_type}", "msa")
        .replace("${version_name}", &mc_args.version)
        .replace("${assets_index_name}", &mc_args.asset_index_name)
        .replace(
            "${game_directory}",
            &*wrap(&canonicalize(&mc_args.game_directory).unwrap()
                .to_string_lossy().to_string()),
        )
        .replace(
            "${assets_root}",
            &*wrap(&canonicalize(&mc_args.assets_directory).unwrap()
                .to_string_lossy().to_string()),
        )
        .replace(
            "${game_assets}",
            &*wrap(&canonicalize(&mc_args.assets_directory).unwrap()
                .to_string_lossy().to_string()),
        )
        .replace("${version_type}", &mc_args.version_type.as_str().to_string())
        .replace("${resolution_width}", &mc_args.resolution.0.to_string())
        .replace("${resolution_height}", &mc_args.resolution.1.to_string())
}

fn parse_jvm_argument(
    mut argument: String,
    jvm_args: &JvmArgs,
) -> String {
    argument.retain(|c| !c.is_whitespace());
    argument
        .replace(
            "${natives_directory}",
            &*wrap(&canonicalize(&jvm_args.natives_path).unwrap()
                .to_string_lossy().to_string()),
        )
        .replace(
            "${library_directory}",
            &*wrap(&canonicalize(&jvm_args.libraries_path).unwrap()
                .to_string_lossy().to_string()),
        )
        .replace("${classpath_separator}", classpath_separator(&jvm_args.java_arch))
        .replace("${launcher_name}", "nexus-launcher")
        .replace("${launcher_version}", "50")
        .replace("${version_name}", &jvm_args.version_name)
        .replace("${classpath}", &jvm_args.class_paths)
}

pub fn classpath_separator(os: &str) -> &'static str {
    match os {
        // Windows is ; and others are :
        /// TODO: Everything is lowercase because of the way it is converted. Not sure if std::env::const::OS returns the other values
        /*"osx"
        | "osxarm64"
        | "linux"
        | "linuxarm32"
        | "linuxarm64"
        | "unknown" => ":",*/
        "windows" | "windowsarm64" => ";",
        _ => ":"
    }
}

pub fn get_classpaths(
    libraries: &[Library],
    client_path: PathBuf,
    libraries_path: PathBuf,
    java_arch: &str,
) -> String {
    let mut class_paths: Vec<String> = Vec::new();

    for library in libraries {
        if let Some(rules) = library.rules.clone() {
            if check_rules(rules) == false { continue };
        }

        if library.include_in_classpath {
            let path = get_path_from_artifact(&library.name);
            match path {
                Ok(p) => class_paths.push(libraries_path.join(p).to_string_lossy().to_string()),
                Err(e) => println!("Could not find library: {}.jar. {:?}", library.name, e),
            }
        }
    }



    class_paths.push(client_path.to_string_lossy().to_string());

    let x = class_paths.join(classpath_separator(java_arch));
    /*println!();
    println!("Classpaths: {}", x);
    println!();*/
    x
}

// Code taken from modrinth's Daedalus
fn get_path_from_artifact(artifact: &str) -> Result<String, String> {
    let name_items = artifact.split(':').collect::<Vec<&str>>();
    let package = name_items.first().ok_or_else(|| {
        format!("Unable to find package for library {}", &artifact)
    })?;
    let name = name_items.get(1).ok_or_else(|| {
        format!("Unable to find name for library {}", &artifact)
    })?;

    if name_items.len() == 3 {
        let version_ext = name_items
            .get(2).unwrap()
            .split('@')
            .collect::<Vec<&str>>();
        let version = version_ext.first().ok_or_else(|| {
            format!("Unable to find version for library {}", &artifact)
        })?;
        let ext = version_ext.get(1);

        Ok(format!(
            "{}/{}/{}/{}-{}.{}",
            package.replace('.', "/"),
            name,
            version,
            name,
            version,
            ext.unwrap_or(&"jar")
        ))
    } else {
        let version = name_items.get(2).ok_or_else(|| {
            format!("Unable to find version for library {}", &artifact)
        })?;

        let data_ext = name_items
            .get(3)
            .ok_or_else(|| {
                format!(
                    "Unable to find data for library {}",
                    &artifact)
            })?
            .split('@')
            .collect::<Vec<&str>>();
        let data = data_ext.first().ok_or_else(|| {
            format!(
                "Unable to find data for library {}",
                &artifact)
        })?;
        let ext = data_ext.get(1);

        Ok(format!(
            "{}/{}/{}/{}-{}-{}.{}",
            package.replace('.', "/"),
            name,
            version,
            name,
            version,
            data,
            ext.unwrap_or(&"jar")
        ))
    }
}

pub fn check_rules(rules: Vec<Rule>) -> bool {
    println!();
    println!();
    let mut allowed: Option<bool> = None;
    for rule in rules {
        println!("{:?}", rule);
        if let Some(os) = &rule.os {
            println!("OS var exists");
            if os.name.clone().is_some_and(|os| os.to_string() == env::consts::OS) {
                println!("OS match");
                println!("{:?}", rule.action);
                allowed = match rule.action {
                    RuleAction::Allow => Some(true),
                    RuleAction::Disallow => Some(false),
                };
            }
            else {
                if allowed.is_none() {
                    println!("Is none OS match");
                    allowed = match rule.action {
                        RuleAction::Allow => Some(false),
                        RuleAction::Disallow => Some(true),
                    };
                }
            }
            if os.arch.clone().is_some_and(|os| os == env::consts::ARCH) {
                println!("ARCH match");
                println!("{:?}", rule.action);
                allowed = match rule.action {
                    RuleAction::Allow => Some(true),
                    RuleAction::Disallow => Some(false),
                };
            }
            else {
                if allowed.is_none() {
                    println!("Is none ARCH match");
                    allowed = match rule.action {
                        RuleAction::Allow => Some(false),
                        RuleAction::Disallow => Some(true),
                    };
                }
            }
        }

        if rule.os.is_none() && allowed.is_none() {
            println!("Is none match");
            allowed = match rule.action {
                RuleAction::Allow => Some(true),
                RuleAction::Disallow => Some(false),
            };
        }
    };

    if !allowed.unwrap_or(true) {
        println!("Returning false");
        return false;
    }
    println!("Returning true");
    true
}

fn wrap(group:&String) -> String {
    format!("{}", group)
}