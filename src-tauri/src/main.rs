// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio, Child};
use serde_json::Value;
use tauri::{command, Manager};
use tauri::api::path::home_dir;
use std::fs;
use std::thread;
use std::env;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;
use tauri::State;
use nix::unistd::Pid;
use nix::sys::signal::{kill, Signal};
use nix::sys::wait::waitpid;

struct ProjectManager(Mutex<HashMap<u32, Child>>);



#[derive(serde::Serialize)]
struct ProjectInfo {
    framework: String,
    runtime: String,
    packages: Vec<Package>,
}

#[derive(serde::Serialize)]
struct Package {
    name: String,
    version: String,
}

#[command]
fn analyze_project(path: String) -> Result<ProjectInfo, String> {
    let package_json_path = Path::new(&path).join("package.json");
    let package_json_content = fs::read_to_string(package_json_path)
        .map_err(|e| format!("Failed to read package.json: {}", e))?;

    let package_json: Value = serde_json::from_str(&package_json_content)
        .map_err(|e| format!("Failed to parse package.json: {}", e))?;

    // let framework = detect_framework(&package_json);
    let (framework, _command) = detect_framework(&package_json);
    let runtime = detect_runtime_version(&path);
    let packages = extract_packages(&package_json);

    Ok(ProjectInfo {
        framework,
        runtime,
        packages,
    })
}
///media/ron-tennyson/Work/Projects/local-node/node_modules/next/dist/bin/next

fn detect_framework(package_json: &Value) -> (String, String) {
    if package_json["dependencies"].get("next").is_some() {
        ("Next.js".to_string(), "node node_modules/next/dist/bin/next dev".to_string())
    } else if package_json["dependencies"].get("react").is_some() {
        ("React".to_string(), "node node_modules/.bin/react-scripts start".to_string())
    } else if package_json["dependencies"].get("vue").is_some() {
        ("Vue.js".to_string(), "node node_modules/.bin/vue-cli-service serve".to_string())
    } else {
        ("Unknown".to_string(), "".to_string())
    }
}
fn detect_runtime_version(path: &str) -> String {
    if Path::new(path).join("pnpm-lock.yaml").exists() {
        "pnpm".to_string()
    } else if Path::new(path).join("yarn.lock").exists() {
        "yarn".to_string()
    } else {
        "npm".to_string()
    }
}

fn extract_packages(package_json: &Value) -> Vec<Package> {
    let mut packages = Vec::new();

    if let Some(deps) = package_json["dependencies"].as_object() {
        for (name, version) in deps {
            packages.push(Package {
                name: name.clone(),
                version: version.as_str().unwrap_or("unknown").to_string(),
            });
        }
    }

    if let Some(dev_deps) = package_json["devDependencies"].as_object() {
        for (name, version) in dev_deps {
            packages.push(Package {
                name: name.clone(),
                version: version.as_str().unwrap_or("unknown").to_string(),
            });
        }
    }

    packages
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
fn start_project_creation(window: tauri::Window, runtime: String, framework: String, project_name: String, location: String) -> Result<(), String> {
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

// #[tauri::command]
// async fn run_command(
//     state: State<'_, RunningState>,
//     window: tauri::Window,
//     project_path: String,
//     command: String
// ) -> Result<CommandStatus, String> {
//     let mut state_guard = state.lock().map_err(|e| e.to_string())?;
//     if state_guard.is_some() {
//         return Err("A command is already running".to_string());
//     }

//     let cmd = if cfg!(target_os = "windows") {
//         Command::new("cmd")
//             .args(&["/C", &command])
//             .current_dir(&project_path)
//             .stdout(Stdio::piped())
//             .stderr(Stdio::piped())
//             .spawn()
//     } else {
//         Command::new("sh")
//             .arg("-c")
//             .arg(&command)
//             .current_dir(&project_path)
//             .stdout(Stdio::piped())
//             .stderr(Stdio::piped())
//             .spawn()
//     };

//     match cmd {
//         Ok(mut child) => {
//             let pid = child.id();
//             let stdout = child.stdout.take().unwrap();
//             let stderr = child.stderr.take().unwrap();

//             *state_guard = Some((child, pid));
//             drop(state_guard);

//             let state_clone = Arc::clone(&state);
//             tauri::async_runtime::spawn(async move {
//                 let stdout_reader = BufReader::new(stdout);
//                 let stderr_reader = BufReader::new(stderr);

//                 for line in stdout_reader.lines().chain(stderr_reader.lines()) {
//                     if let Ok(line) = line {
//                         let _ = window.emit("command_output", &line);
//                     }
//                     if state_clone.lock().unwrap().is_none() {
//                         break;
//                     }
//                 }

//                 let mut state_guard = state_clone.lock().unwrap();
//                 if let Some((child, _)) = state_guard.as_mut() {
//                     let _ = child.wait();
//                 }
//                 *state_guard = None;
//                 let _ = window.emit("command_finished", CommandStatus { is_running: false, pid: None });
//             });

//             Ok(CommandStatus { is_running: true, pid: Some(pid) })
//         }
//         Err(e) => {
//             Err(e.to_string())
//         }
//     }
// }

#[command]
fn start_project(
    project_path: String,
    window: tauri::Window,
    state: State<'_, ProjectManager>,
) -> Result<u32, String> {
    // Read package.json
    let package_json_path = format!("{}/package.json", project_path);
    let package_json: Value = serde_json::from_str(&std::fs::read_to_string(package_json_path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;

    let (_framework, command) = detect_framework(&package_json);
    
    if command.is_empty() {
        return Err("Unsupported framework".to_string());
    }

    // Split the command into executable and arguments
    let mut command_parts = command.split_whitespace();
    let executable = command_parts.next().ok_or("Failed to parse command")?;
    let args: Vec<&str> = command_parts.collect();

    // Spawn the child process directly
    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &command])
            .current_dir(&project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    } else {
        Command::new(executable)
            .args(&args)
            .current_dir(&project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
    }.map_err(|e| e.to_string())?;

    let pid = child.id();

    // Stream output
    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
    
    let window_clone = window.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                window.emit("project-output", (pid, line)).unwrap();
            }
        }
    });

    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                window_clone.emit("project-error", (pid, line)).unwrap();
            }
        }
    });
    
    // Store the child process
    state.0.lock().unwrap().insert(pid, child);
    
    Ok(pid)
}

#[command]
fn close_project(state: State<'_, ProjectManager>, pid: u32) -> Result<(), String> {
    let mut projects = state.0.lock().unwrap();
    if projects.remove(&pid).is_some() {
        // Send SIGTERM to the entire process group
        kill(Pid::from_raw(pid as i32), Signal::SIGTERM).map_err(|e| e.to_string())?;
        
        // Wait for the process group to exit
        loop {
            match waitpid(Pid::from_raw(pid as i32), None) {
                Ok(_) => break,
                Err(e) => return Err(e.to_string()),
            }
        }
    }
    Ok(())
}



fn main() {
    // Initialize the state
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .manage(ProjectManager(Mutex::new(HashMap::new()))) // Manage the state within Tauri
        .invoke_handler(tauri::generate_handler![
            create_local_projects_folder,
            start_project_creation,
            create_project,
            launch_vscode,
            open_file_explorer,
            detect_runtime,
            analyze_project,
            start_project,
            close_project,
            
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
