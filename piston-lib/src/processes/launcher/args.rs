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
}


pub fn format_arguments(arguments: Option<HashMap<ArgumentType, Vec<Argument>>>, legacy_minecraft_args: Option<String>, mc_args: MinecraftArgs, jvm_args: JvmArgs, main_class: String) -> Vec<String> {
    if let Some(legacy_minecraft_args) = legacy_minecraft_args {
        return format_arguments_legacy(legacy_minecraft_args, mc_args, &jvm_args, main_class);
    }

    if let Some(arguments) = arguments {
        let mut formatted_args: Vec<String> = Vec::new();

        // JVM args
        formatted_args.append(&mut get_formatted_jvm_arguments(arguments.clone(), jvm_args));

        // Main class
        formatted_args.push(main_class);

        // Minecraft args
        formatted_args.append(&mut get_formatted_mc_arguments(arguments, mc_args));

        return formatted_args;
    }

    panic!("No arguments provided");
}

fn format_arguments_legacy(arguments: String, mc_args: MinecraftArgs, jvm_args: &JvmArgs, main_class: String) -> Vec<String> {
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
        args.push(parse_mc_argument(&arg.to_string(), &mc_args));
    }
    args
}


pub fn check_rules(rules: &Vec<Rule>) -> bool {
    let mut allowed: Option<bool> = None;
    for rule in rules {
        println!("Rule: {:?}", rule);
        if let Some(os) = &rule.os {
            if os.name.clone().is_some_and(|os| os.to_string() == env::consts::OS.replace("macos", "osx")) {
                allowed = match rule.action {
                    RuleAction::Allow => Some(true),
                    RuleAction::Disallow => Some(false),
                };
            }
            else {
                if allowed.is_none() {
                    allowed = match rule.action {
                        RuleAction::Allow => Some(false),
                        RuleAction::Disallow => Some(true),
                    };
                }
            }

            if os.arch.clone().is_some_and(|arch| arch == env::consts::ARCH) {
                allowed = match rule.action {
                    RuleAction::Allow => Some(true),
                    RuleAction::Disallow => Some(false),
                };
            }
            else {
                if allowed.is_none() {
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
        println!("Rule check is false");
        return false;
    }
    println!("Rule check is true");
    true
}



fn get_formatted_jvm_arguments(arguments: HashMap<ArgumentType, Vec<Argument>>, set_args: JvmArgs) -> Vec<String> {
    if let Some(jvm_args) = arguments.get(&ArgumentType::Jvm) {
        let mut formatted_jvm_args: Vec<String> = Vec::new();

        for arg in jvm_args {
            match arg {
                Normal(arg) => {
                    formatted_jvm_args.push(parse_jvm_argument(arg, &set_args));
                }
                Ruled { rules, value } => {
                    if check_rules(rules) == true {
                        match value {
                            Single(v) => {
                                formatted_jvm_args.push(parse_jvm_argument(v, &set_args));
                            },
                            Many(v) => {
                                for arg in v {
                                    formatted_jvm_args.push(parse_jvm_argument(arg, &set_args));
                                }
                            }
                        }
                    };
                }
            }
        }

        println!("Formatted jvm arguments: {:?}", formatted_jvm_args);
        return formatted_jvm_args;
    }

    Vec::new()
}

fn get_formatted_mc_arguments(arguments: HashMap<ArgumentType, Vec<Argument>>, set_args: MinecraftArgs) -> Vec<String> {
    if let Some(args) = arguments.get(&ArgumentType::Game) {
        let mut formatted_mc_args: Vec<String> = Vec::new();

        for arg in args {
            match arg {
                Normal(arg) => {
                    formatted_mc_args.push(parse_mc_argument(&arg, &set_args));
                }
                Ruled { rules, value } => {
                    if let Some(feature) = &rules[0].features {
                        if feature.has_custom_resolution.is_some() {
                            match value {
                                Many(v) => {
                                    for arg in v {
                                        formatted_mc_args.push(parse_mc_argument(arg, &set_args));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        return formatted_mc_args;
    }

    Vec::new()
}



fn parse_jvm_argument(argument: &String, jvm_args: &JvmArgs) -> String {
    let new = argument
        .replace(
            "${natives_directory}",
            &*wrap(&canonicalize(&jvm_args.natives_path).unwrap()
                .into_os_string().into_string().unwrap()),
        )
        .replace(
            "${library_directory}",
            &*wrap(&canonicalize(&jvm_args.libraries_path).unwrap()
                .into_os_string().into_string().unwrap()),
        )
        /*.replace("${classpath_separator}", classpath_separator(env::consts::OS))*/
        .replace("${launcher_name}", "nexus-launcher")
        .replace("${launcher_version}", "50")
        .replace("${version_name}", &jvm_args.version_name)
        .replace("${classpath}", &jvm_args.class_paths);

    if &new == argument && argument.contains("${") {
        eprintln!("There was no match for the argument \"{}\"", argument);
    }

    new
}



fn parse_mc_argument(argument: &String, mc_args: &MinecraftArgs) -> String {
    let new = argument.replace("${accessToken}", &mc_args.access_token)
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
            wrap(&canonicalize(&mc_args.game_directory).unwrap()
                .into_os_string().into_string().unwrap()).as_str(),
        )
        .replace(
            "${assets_root}",
            wrap(&canonicalize(&mc_args.assets_directory).unwrap()
                .into_os_string().into_string().unwrap()).as_str(),
        )
        .replace(
            "${game_assets}",
            wrap(&canonicalize(&mc_args.assets_directory).unwrap()
                .into_os_string().into_string().unwrap()).as_str(),
        )
        .replace("${version_type}", &mc_args.version_type.as_str().to_string())
        .replace("${resolution_width}", &mc_args.resolution.0.to_string())
        .replace("${resolution_height}", &mc_args.resolution.1.to_string());

    if &new == argument && argument.contains("${") {
        eprintln!("There was no match for the argument \"{}\"", argument);
    }

    new
}


fn wrap(txt: &String) -> String {
    format!("{}", txt)
}




pub fn get_classpaths(
    libraries: &[Library],
    client_path: PathBuf,
    libraries_path: PathBuf,
) -> String {
    let mut class_paths: Vec<String> = Vec::new();
    println!("Grabbing class paths");

    for library in libraries {
        if let Some(rules) = &library.rules {
            println!("-- Checking rules for {}", library.name);
            if check_rules(rules) == false { continue };
        }

        if library.include_in_classpath {
            let path = get_path_from_artifact(&library.name);
            match path {
                Ok(p) => {
                    let formatted_p = libraries_path.join(p).to_string_lossy().to_string();
                    println!("-- Adding {} to classpath", &formatted_p);
                    if !PathBuf::from(&formatted_p).exists() {
                        println!("Library was not correctly downloaded!: {}.jar", library.name);
                    }

                    if !class_paths.contains(&formatted_p) {
                        class_paths.push(formatted_p);
                    }
                    else {
                        println!("Class path matches. But path is already in the list");
                    }
                },
                Err(e) => println!("Could not find library: {}.jar. {:?}", library.name, e),
            }
        }
    }



    class_paths.push(client_path.to_string_lossy().to_string());

    let x = class_paths.join(if env::consts::OS == "windows" {";"} else {":"} );
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