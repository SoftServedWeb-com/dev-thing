// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod utils;
mod project_manager;
mod manage_packages;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::project_manager::ProjectManager;

fn main() {
    let _ = fix_path_env::fix(); // to get the PATH environment variable
    tauri::Builder::default()
        .manage(ProjectManager(Mutex::new(HashMap::new()))) // Manage the state within Tauri
        .invoke_handler(tauri::generate_handler![
            commands::create_local_projects_folder,
            commands::start_project_creation,
            commands::launch_ide,
            commands::open_file_explorer,
            commands::detect_runtime,
            commands::analyze_project,
            commands::start_project,
            commands::close_project,
            manage_packages::install_dependency,
            manage_packages::update_dependency,
            manage_packages::delete_dependency,
            manage_packages::reinstall_dependencies,
            commands::update_project_path,
            commands::delete_site,
        ])
        .setup(|_app| 
            Ok(())
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}