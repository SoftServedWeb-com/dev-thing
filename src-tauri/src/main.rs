// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::windows::process::CommandExt; // Add this import for Windows-specific extensions
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio, Child};
use serde_json::Value;
use tauri::{command, Window};
use tauri::api::path::home_dir;
use std::fs;
use std::thread;
use std::env;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;
use tauri::State;
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
#[cfg(target_os = "windows")]
use winapi::um::winnt::{PROCESS_TERMINATE, HANDLE};
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::DWORD;

#[cfg(unix)]
use nix::unistd::Pid;
#[cfg(unix)]
use nix::sys::signal::{kill, Signal};

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
    let (framework, _command) = detect_framework(&package_json, "");
    let runtime = detect_runtime_version(&path);
    let packages = extract_packages(&package_json);

    Ok(ProjectInfo {
        framework,
        runtime,
        packages,
    })
}
///media/ron-tennyson/Work/Projects/local-node/node_modules/next/dist/bin/next

fn detect_framework(package_json: &Value, path: &str) -> (String, String) {
    if package_json["dependencies"].get("next").is_some() {
        let path_to_dev;
        if cfg!(target_os = "windows") {
            path_to_dev = format!("{}\\node_modules\\next\\dist\\bin\\next dev", path)
        }
        else {
            path_to_dev = format!("{}/node_modules/next/dist/bin/next dev", path)
        }
        ("Next.js".to_string(), format!("node {}", path_to_dev).to_string())
    } else if package_json["dependencies"].get("react").is_some() {
        let path_to_dev;
        if cfg!(target_os = "windows") {
            path_to_dev = format!("{}\\node_modules\\react-scripts\\scripts\\start.js", path)
        }
        else {
            path_to_dev = format!("{}/node_modules/react-scripts/scripts/start.js", path)
        }
        ("React".to_string(), format!("node {}", path_to_dev).to_string())
    } else if package_json["dependencies"].get("vue").is_some() {
        let path_to_dev;
        if cfg!(target_os = "windows") {
            path_to_dev = format!("{}\\node_modules\\vue-cli-service\\bin\\vue-cli-service.js", path)
        }
        else {
            path_to_dev = format!("{}/node_modules/vue-cli-service/bin/vue-cli-service.js", path)
        }
        ("Vue.js".to_string(), format!("node {}", path_to_dev).to_string())
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
fn launch_ide(project_path: String, ide: String) -> Result<(), String> {
    let os = env::consts::OS;

    match os {
        "windows" => {
            Command::new("cmd")
                .args(&["/C", &ide, &project_path])
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .spawn()
                .map_err(|e| format!("Failed to launch {}: {}", ide, e))?;
        },
        "macos" | "linux" => {
            Command::new(&ide)
                .arg(&project_path)
                .spawn()
                .map_err(|e| format!("Failed to launch {}: {}", ide, e))?;
        },
        _ => {
            return Err("Unsupported operating system".into());
        }
    }

    Ok(())
}

#[command]
fn open_file_explorer(project_path: String) -> Result<(), String> {
    let os = env::consts::OS;
    
    match os {
        "windows" => {
            Command::new("explorer")
                .arg(&project_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
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
    let create_path;
    if cfg!(target_os = "windows") {
         create_path = format!("{}\\{}", location, project_name);
    } else {
         create_path = format!("{}/{}", location, project_name);
    }
    thread::spawn(move || {
        window.emit("creation_status", "Starting project creation...").unwrap();
        
        let cmd = match (runtime.as_str(), framework.as_str()) {
            ("pnpm", "next.js") => {
                if cfg!(target_os = "windows") {
                    
                    format!("pnpx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", create_path)
                } else {
                    format!("pnpx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", create_path)
                }
            }
            ("npm", "next.js") => {
                if cfg!(target_os = "windows") {
                    format!("npx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", create_path)
                } else {
                    format!("npx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", create_path)
                }
            }
            ("bun", "next.js") => {
                if cfg!(target_os = "windows") {
                    format!("bunx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", create_path)
                } else {
                    format!("bunx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", create_path)
                }
            }
            _ => {
                window.emit("creation_status", "Error: Unsupported runtime or framework").unwrap();
                return;
            }
        };

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &cmd])
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
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
fn create_local_projects_folder(folder_name: Option<String>, path: Option<String>) -> Result<String, String> {
    let home_path = home_dir().ok_or("Could not determine home directory")?;
    let projects_path = match (folder_name, path) {
        (Some(name), Some(p)) => Path::new(&p).join(name),
        (Some(name), None) => home_path.join(name),
        (None, Some(p)) => Path::new(&p).to_path_buf(),
        (None, None) => home_path.join("Local-Projects"),
    };

    if !projects_path.exists() {
        fs::create_dir_all(&projects_path).map_err(|err| err.to_string())?;
    }

    Ok(projects_path.to_str().unwrap().to_string())
}


#[tauri::command]
fn start_project(
    project_path: String,
    window: tauri::Window,
    state: State<'_, ProjectManager>,
) -> Result<u32, String> {
    // Read package.json
    let package_json_path;
    if cfg!(target_os = "windows") {
        package_json_path = format!("{}\\package.json", project_path);
    }
    else {
        package_json_path = format!("{}/package.json", project_path);
    }
    let package_json: Value = serde_json::from_str(&std::fs::read_to_string(package_json_path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;

    let (_framework, command) = detect_framework(&package_json, &project_path);
    
    if command.is_empty() {
        return Err("Unsupported framework".to_string());
    }

    let mut command_parts = command.split_whitespace();
    let executable = command_parts.next().ok_or("Failed to parse command")?;
    let args: Vec<&str> = command_parts.collect();

    println!("Command: {}", command);
    // Spawn the child process
    let mut child = if cfg!(target_os = "windows") {
        Command::new(executable)
            .args(&args)
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
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

#[cfg(target_os = "windows")]
fn terminate_process(pid: u32) -> Result<(), String> {
    unsafe {
        let process_handle: HANDLE = OpenProcess(PROCESS_TERMINATE, 0, pid as DWORD);
        if process_handle.is_null() {
            println!("Failed to open process");
            return Err("Failed to open process".to_string());
        }

        if TerminateProcess(process_handle, 1) == 0 {
            CloseHandle(process_handle);
            println!("Failed to terminate process");
            return Err("Failed to terminate process".to_string());
        }

        CloseHandle(process_handle);
    }
    Ok(())
}

#[cfg(unix)]
fn terminate_process(pid: u32) -> Result<(), String> {
    kill(Pid::from_raw(pid as i32), Signal::SIGTERM).map_err(|e| e.to_string())
}


#[tauri::command]
fn close_project(state: State<'_, ProjectManager>, pid: u32) -> Result<(), String> {
    let mut projects = state.0.lock().unwrap();
    if projects.remove(&pid).is_some() {
        terminate_process(pid)?;
    }
    Ok(())
}

#[command]
fn install_dependency(
    window: Window,
    project_path: String,
    runtime: String,
    dependency: String,
    version: Option<String>,
) -> Result<(), String> {
    let versioned_dependency = if let Some(ver) = version {
        format!("{}@{}", dependency, ver)
    } else {
        dependency
    };
    let cmd = match runtime.as_str() {
        "pnpm" => format!("pnpm add {}", versioned_dependency),
        "npm" => format!("npm install {}", versioned_dependency),
        "yarn" => format!("yarn add {}", versioned_dependency),
        "bun" => format!("bun add {}", versioned_dependency),
        _ => return Err("Unsupported runtime".to_string()),
    };
    println!("Executing command: {}", cmd);
    thread::spawn(move || {
        window.emit("install_status", "Installing dependency...").unwrap();
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &cmd])
                .current_dir(&project_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .current_dir(&project_path)
                .output()
        };
        match output {
            Ok(output) => {
                println!("Command executed. Exit status: {}", output.status);
                println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                let message = format!("{} installed successfully!", String::from_utf8_lossy(&output.stdout));
                if output.status.success() {
                    window.emit("install_status", message).unwrap();
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    let status_message = if error_message.trim().is_empty() {
                        format!("{} Installation failed. Exit code: {}. Check stdout for details.", message, output.status.code().unwrap_or(-1))
                    } else {
                        format!("Error: {}", error_message)
                    };
                    println!("{}", status_message);
                    window.emit("install_status", status_message).unwrap();
                }
            },
            Err(e) => {
                let error_message = format!("Failed to execute command: {}", e);
                println!("{}", error_message);
                window.emit("install_status", error_message).unwrap();
            }
        }
    });
    Ok(())
}

#[command]
fn update_dependency(
    window: Window,
    project_path: String,
    runtime: String,
    dependency: String,
    version: Option<String>,
) -> Result<(), String> {
    let cmd = if let Some(ver) = version {
        match runtime.as_str() {
            "pnpm" => format!("pnpm add {}@{}", dependency, ver),
            "npm" => format!("npm install {}@{}", dependency, ver),
            "yarn" => format!("yarn add {}@{}", dependency, ver),
            "bun" => format!("bun add {}@{}", dependency, ver),
            _ => return Err("Unsupported runtime".to_string()),
        }
    } else {
        match runtime.as_str() {
            "pnpm" => format!("pnpm update {}", dependency),
            "npm" => format!("npm update {}", dependency),
            "yarn" => format!("yarn upgrade {}", dependency),
            "bun" => format!("bun update {}", dependency),
            _ => return Err("Unsupported runtime".to_string()),
        }
    };

    println!("Executing command: {}", cmd);
    thread::spawn(move || {
        window.emit("update_status", "Updating dependency...").unwrap();
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &cmd])
                .current_dir(&project_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .current_dir(&project_path)
                .output()
        };
        match output {
            Ok(output) => {
                println!("Command executed. Exit status: {}", output.status);
                println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                let message = format!("{} updated successfully!", String::from_utf8_lossy(&output.stdout));
                if output.status.success() {
                    window.emit("update_status", message).unwrap();
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    let status_message = if error_message.trim().is_empty() {
                        format!("{} Update failed. Exit code: {}. Check stdout for details.", message, output.status.code().unwrap_or(-1))
                    } else {
                        format!("Error: {}", error_message)
                    };
                    println!("{}", status_message);
                    window.emit("update_status", status_message).unwrap();
                }
            },
            Err(e) => {
                let error_message = format!("Failed to execute command: {}", e);
                println!("{}", error_message);
                window.emit("update_status", error_message).unwrap();
            }
        }
    });
    Ok(())
}

#[command]
fn delete_dependency(
    window: Window,
    project_path: String,
    runtime: String,
    dependency: String,
) -> Result<(), String> {
    let cmd = match runtime.as_str() {
        "pnpm" => format!("pnpm remove {}", dependency),
        "npm" => format!("npm uninstall {}", dependency),
        "yarn" => format!("yarn remove {}", dependency),
        "bun" => format!("bun remove {}", dependency),
        _ => return Err("Unsupported runtime".to_string()),
    };
    println!("Executing command: {}", cmd);
    thread::spawn(move || {
        window.emit("delete_status", "Deleting dependency...").unwrap();
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &cmd])
                .current_dir(&project_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .current_dir(&project_path)
                .output()
        };
        match output {
            Ok(output) => {
                println!("Command executed. Exit status: {}", output.status);
                println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                let message = format!("{} deleted successfully!", String::from_utf8_lossy(&output.stdout));
                if output.status.success() {
                    window.emit("delete_status", message).unwrap();
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    let status_message = if error_message.trim().is_empty() {
                        format!("{} Deletion failed. Exit code: {}. Check stdout for details.", message, output.status.code().unwrap_or(-1))
                    } else {
                        format!("Error: {}", error_message)
                    };
                    println!("{}", status_message);
                    window.emit("delete_status", status_message).unwrap();
                }
            },
            Err(e) => {
                let error_message = format!("Failed to execute command: {}", e);
                println!("{}", error_message);
                window.emit("delete_status", error_message).unwrap();
            }
        }
    });
    Ok(())
}

#[command]
fn update_project_path(new_path: String) -> Result<String, String> {
    let home_path = home_dir().ok_or("Could not determine home directory")?;
    let projects_path = if new_path.is_empty() {
        home_path.join("Local-Projects")
    } else {
        home_path.join(new_path)
    };

    if !projects_path.exists() {
        fs::create_dir_all(&projects_path).map_err(|err| err.to_string())?;
    }

    Ok(projects_path.to_str().unwrap().to_string())
}

#[command]
fn reinstall_dependencies(
    window: Window,
    project_path: String,
    runtime: String,
) -> Result<(), String> {
    let cmd = match runtime.as_str() {
        "pnpm" => "pnpm install".to_string(),
        "npm" => "npm install".to_string(),
        "yarn" => "yarn install".to_string(),
        "bun" => "bun install".to_string(),
        _ => return Err("Unsupported runtime".to_string()),
    };
    println!("Executing command: {}", cmd);
    thread::spawn(move || {
        window.emit("reinstall_status", "Reinstalling dependencies...").unwrap();
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &cmd])
                .current_dir(&project_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .current_dir(&project_path)
                .output()
        };
        match output {
            Ok(output) => {
                println!("Command executed. Exit status: {}", output.status);
                println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                let message = format!("Dependencies reinstalled successfully!\n{}", String::from_utf8_lossy(&output.stdout));
                if output.status.success() {
                    window.emit("reinstall_status", message).unwrap();
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    let status_message = if error_message.trim().is_empty() {
                        format!("Reinstallation failed. Exit code: {}. Check stdout for details.", output.status.code().unwrap_or(-1))
                    } else {
                        format!("Error: {}", error_message)
                    };
                    println!("{}", status_message);
                    window.emit("reinstall_status", status_message).unwrap();
                }
            },
            Err(e) => {
                let error_message = format!("Failed to execute command: {}", e);
                println!("{}", error_message);
                window.emit("reinstall_status", error_message).unwrap();
            }
        }
    });
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
            launch_ide,
            open_file_explorer,
            detect_runtime,
            analyze_project,
            start_project,
            close_project,
            install_dependency,
            update_dependency,
            delete_dependency,
            update_project_path,
            reinstall_dependencies,
        ])
        .setup(|_app| 
            Ok(())
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
