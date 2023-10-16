// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod macos;
mod fs;
mod config;
mod processes;
pub mod services;
mod handler;

use std::collections::VecDeque;
use std::sync::Mutex;
use specta::{collect_types};
use tauri_specta::{ts};
use tauri::Manager;
use tauri::RunEvent::WindowEvent;
use piston_lib::data_structures::game::mojang_version_manifest::VersionManifestRoot;
use crate::config::*;
use crate::config::structs::instances::{NexusInstance, InstancesToml};
use crate::processes::instance::{get_versions, install_instance};
use crate::services::install_service::InstallationService;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


/// Initialize the app
fn main() {
    export_bindings();
    /*write_instance_toml(InstancesToml {
        instance: vec!(NexusInstance::default()),
    });*/

    let builder = tauri::Builder::default()
        .manage(InstallationService::new())
        .invoke_handler(tauri::generate_handler![
            get_app_config_dir_path,
            get_app_config,
            write_app_config,
            get_instances_toml,
            write_instance_toml,
            install_instance,
            get_versions,
        ])
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            {
                use macos::window_ext::WindowExt;

                win.set_transparent_titlebar(true);
                win.position_traffic_lights(30.0, 30.0);
            }

            Ok(())
        });

    #[cfg(target_os = "macos")]
    {
        use tauri::WindowEvent;
        builder.on_window_event(|e| {
            use macos::window_ext::WindowExt;
            if let WindowEvent::Resized(..) = e.event() {
                let win = e.window();
                win.position_traffic_lights(30.0, 30.0);
            }
        });
    }

    builder.run(tauri::generate_context!())
        .expect("error while running tauri application");
}


/// Export the tauri-specta bindings
fn export_bindings() {
    // TODO In the future add multiple files with bindings for organisation

    #[cfg(debug_assertions)]
    match ts::export(collect_types![
        install_instance,
        get_versions,
    ], "../src/bindings.ts")
    {
        Ok(_) => println!("Export to bindings.ts successful"),
        Err(e) => eprintln!("Error during export to bindings.ts: {:?}", e),
    };

    #[cfg(debug_assertions)]
    match ts::export(collect_types![
        get_app_config_dir_path,
        get_app_config,
        write_app_config,
        get_instances_toml,
        write_instance_toml,
    ], "../src/config.ts")
    {
        Ok(_) => println!("Export to config.ts successful"),
        Err(e) => eprintln!("Error during export to config.ts: {:?}", e),
    };;
}

