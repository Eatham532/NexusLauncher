// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod macos;
pub mod fs;
pub mod config;
pub mod processes;
pub mod services;
pub mod handlers;

use std::collections::VecDeque;
use std::fs::create_dir_all;
use std::sync::Mutex;
use specta::{collect_types};
use tauri_specta::{ts};
use tauri::Manager;
use tauri::RunEvent::WindowEvent;
use toml::value::Array;
use piston_lib::data_structures::game::metadata::mojang_version_manifest::McVersionManifest;
use piston_lib::data_structures::game::metadata::piston_version_manifest::{LoaderVersion, PistonMetadata};
use piston_lib::processes::launcher::installation::vanilla::get_version_manifest;
use piston_lib::processes::launcher::installation::versioning::versioning::generate_versions_metadata;
use crate::config::*;
use crate::processes::auth::{cancel_auth, start_login};
use crate::processes::instance::{get_versions, install_instance, launch_instance, delete_instance};
use crate::processes::user::{get_pfp_path, change_active_user, logout_user, pre_download_user_icons};
use crate::services::install_service::InstallationService;



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


/// Initialize the app
fn main() {
    /*do_pre_work();

    std::thread::spawn(move || {
        do_startup_work();
    });*/

    /*let builder = tauri::Builder::default()*/
        /*.manage(InstallationService::new())
        .invoke_handler(tauri::generate_handler![
            get_appdata_dir_path,
            get_app_config,
            write_app_config,
            get_instances_toml,
            write_instance_toml,
            install_instance,
            get_versions,
            launch_instance,
            delete_instance,
            start_login,
            cancel_auth,
            get_pfp_path,
            get_users,
            change_active_user,
            logout_user,
        ])*/
        /*.setup(|app| {
            let win = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            {
                use macos::window_ext::WindowExt;

                win.set_transparent_titlebar(true);
                win.position_traffic_lights(30.0, 30.0);
            }

            Ok(())
        })*/;

    /*#[cfg(target_os = "macos")]
    {
        use tauri::WindowEvent;
        builder.on_window_event(|e| {
            use macos::window_ext::WindowExt;
            if let WindowEvent::Resized(..) = e.event() {
                let win = e.window();
                win.position_traffic_lights(30.0, 30.0);
            }
        });
    }*/

    builder.run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    
}


fn do_pre_work() {
    pre_download_user_icons();

    #[cfg(debug_assertions)]
    export_bindings();
}

fn do_startup_work() {
    // Generate version Metadata
    println!("Doing startup work");
    let metadata_path = get_cache_path().join("version_metadata.json");
    println!("Generating metadata");
    /*let meta = generate_versions_metadata();
    let json = serde_json::to_string(&meta).unwrap();
    std::fs::write(metadata_path, json).unwrap();*/
    
    
    println!("Finished generating metadata");
}



/// Export the tauri-specta bindings
fn export_bindings() {
    // TODO In the future add multiple files with bindings for organisation
    let path = "../src/scripts/rust";

    match ts::export(collect_types![
        get_instances_toml,
        write_instance_toml,
        install_instance,
        get_versions,
        launch_instance,
        delete_instance,
    ], format!("{}/instances.ts", path))
    {
        Ok(_) => println!("Export to instances.ts successful"),
        Err(e) => eprintln!("Error during export to bindings.ts: {:?}", e),
    };

    match ts::export(collect_types![
        start_login,
        cancel_auth,
    ], format!("{}/auth.ts", path))
    {
        Ok(_) => println!("Export to auth.ts successful"),
        Err(e) => eprintln!("Error during export to bindings.ts: {:?}", e),
    };

    match ts::export(collect_types![
        get_appdata_dir_path,
        get_app_config,
        write_app_config,
    ], format!("{}/config.ts", path))
    {
        Ok(_) => println!("Export to config.ts successful"),
        Err(e) => eprintln!("Error during export to config.ts: {:?}", e),
    };

    match ts::export(collect_types![
        get_pfp_path,
        get_users,
        change_active_user,
        logout_user,
    ], format!("{}/user.ts", path))
    {
        Ok(_) => println!("Export to user.ts successful"),
        Err(e) => eprintln!("Error during export to user.ts: {:?}", e),
    };
}