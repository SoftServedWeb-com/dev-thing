// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::{command, Manager};
use tauri::api::path::home_dir;
use std::fs;
use std::thread;
use std::env;


#[command]
fn run_command(project_path: String, command: String) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", "cmd.exe", "/K", &command])
            .current_dir(project_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg("-a")
            .arg("Terminal")
            .arg(project_path)
            .arg(format!("; {}", command))
            .spawn()
            .map_err(|e| e.to_string())?;
    } else if cfg!(target_os = "linux") {
        Command::new("gnome-terminal")
            .arg("--")
            .arg("sh")
            .arg("-c")
            .arg(format!("cd {} && {}", project_path, command))
            .spawn()
            .map_err(|e| e.to_string())?;
    } else {
        return Err("Unsupported operating system".to_string());
    }

    Ok(())
}

#[command]
fn detect_runtime(project_path: String) -> Result<String, String> {
    if fs::metadata(format!("{}/pnpm-lock.yaml", project_path)).is_ok() {
        Ok("pnpm".to_string())
    } else if fs::metadata(format!("{}/bun.lockb", project_path)).is_ok() {
        Ok("bun".to_string())
    } else if fs::metadata(format!("{}/package-lock.json", project_path)).is_ok() {
        Ok("npm".to_string())
    } else {
        Err("No supported package manager lock file found".to_string())
    }
}

#[command]
fn launch_vscode(project_path: String) -> Result<(), String> {
    Command::new("code")
        .arg(project_path)
        .spawn()
        .map_err(|e| format!("Failed to launch VS Code: {}", e))?;

    Ok(())
}

#[command]
fn open_file_explorer(project_path: String) -> Result<(), String> {
    let os = env::consts::OS;
    
    match os {
        "windows" => {
            Command::new("explorer")
                .args(["/select,", &project_path])
                .spawn()
                .map_err(|e| format!("Failed to open file explorer: {}", e))?;
        },
        "macos" => {
            Command::new("open")
                .args(["-R", &project_path])
                .spawn()
                .map_err(|e| format!("Failed to open file explorer: {}", e))?;
        },
        "linux" => {
            // Try xdg-open first, fall back to dbus-send
            if let Err(_) = Command::new("xdg-open")
                .arg(&project_path)
                .spawn() {
                Command::new("dbus-send")
                    .args([
                        "--session",
                        "--dest=org.freedesktop.FileManager1",
                        "--type=method_call",
                        "/org/freedesktop/FileManager1",
                        "org.freedesktop.FileManager1.ShowItems",
                        &format!("array:string:file://{}", project_path),
                        "string:\"\""
                    ])
                    .spawn()
                    .map_err(|e| format!("Failed to open file explorer: {}", e))?;
            }
        },
        _ => {
            return Err("Unsupported operating system".into());
        }
    }

    Ok(())
}

#[tauri::command]
fn start_project_creation(
    window: tauri::Window,
    runtime: String,
    framework: String,
    project_name: String,
    location: String,
) -> Result<(), String> {
    thread::spawn(move || {
        window.emit("creation_status", "Starting project creation...").unwrap();

        let cmd = match (runtime.as_str(), framework.as_str()) {
            ("pnpm", "next.js") => {
                format!("cd \"{}\" && pnpx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", location, project_name)
            }
            ("npm", "next.js") => {
                format!("cd \"{}\" && npx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", location, project_name)
            }
            ("bun", "next.js") => {
                format!("cd \"{}\" && bunx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", location, project_name)
            }
            _ => {
                window.emit("creation_status", "Error: Unsupported runtime or framework").unwrap();
                return;
            }
        };

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &cmd])
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
        };

        match output {
            Ok(output) => {
                if output.status.success() {
                    window.emit("creation_status", "Project created successfully!").unwrap();
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    window.emit("creation_status", format!("Error: {}", error_message)).unwrap();
                }
            },
            Err(e) => {
                window.emit("creation_status", format!("Failed to execute command: {}", e)).unwrap();
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn create_local_projects_folder() -> Result<String, String> {
    if let Some(home_path) = home_dir() {
        let projects_path = home_path.join("Local-Projects");
        if !projects_path.exists() {
            fs::create_dir_all(&projects_path).map_err(|err| err.to_string())?;
        }
        
        Ok(projects_path.to_str().unwrap().to_string())

    } else {
        Err("Could not determine home directory".into())
    }
}

#[command]
fn create_project(runtime: &str, framework: &str, project_name: &str, location: &str) -> Result<String, String> {
  let cmd = match (runtime, framework) {
      ("pnpm", "next.js") => {
          format!("cd {} && pnpx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", location, project_name)
      }
      ("npm", "next.js") => {
          format!("cd {} && npx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", location, project_name)
      }
      ("bun", "next.js") => {
          format!("cd {} && bunx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", location, project_name)
      }
      // Add other frameworks and commands as needed
      _ => return Err("Unsupported runtime or framework".into()),
  };

  let output = Command::new("sh")
      .arg("-c")
      .arg(cmd)
      .output()
      .expect("Failed to execute command");
  print!("{}", String::from_utf8_lossy(&output.stdout));
  if output.status.success() {
      Ok(String::from_utf8_lossy(&output.stdout).to_string())
  } else {
      Err(String::from_utf8_lossy(&output.stderr).to_string())
  }
}


fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_local_projects_folder,
            start_project_creation,
            create_project,
            launch_vscode,
            open_file_explorer,
            detect_runtime,
            run_command
        ])
        .setup(|app| {
            // Create the "Local Projects" folder on startup
            let main_window = app.get_window("main").unwrap();
            let projects_path = create_local_projects_folder().unwrap();

            // Save the path to local storage (you can handle this via Tauri or JavaScript later)
            main_window.eval(&format!(
                "localStorage.setItem('projectsPath', '{}');",
                projects_path
            )).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}