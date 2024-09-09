use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::fs;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use tauri::{command, State};
use serde_json::Value;
use crate::project_manager::terminate_process;
use crate::utils::{execute_command, parse_command};
use crate::project_manager::ProjectManager;
use tauri::api::path::home_dir;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(serde::Serialize)]
pub struct ProjectInfo {
    framework: String,
    runtime: String,
    packages: Vec<Package>,
}

#[derive(serde::Serialize)]
pub struct Package {
    name: String,
    version: String,
}

// detecting framework and runtime
#[command]
pub fn analyze_project(path: String) -> Result<ProjectInfo, String> {
    let package_json_path = Path::new(&path).join("package.json");
    let package_json_content = fs::read_to_string(package_json_path)
        .map_err(|e| format!("Failed to read package.json: {}", e))?;

    let package_json: Value = serde_json::from_str(&package_json_content)
        .map_err(|e| format!("Failed to parse package.json: {}", e))?;

    let (framework, _command) = detect_framework(&package_json, "");
    let runtime = detect_runtime_version(&path);
    let packages = extract_packages(&package_json);

    Ok(ProjectInfo {
        framework,
        runtime,
        packages,
    })
}
// yarn plug and play doesn't have node modules folder
pub fn detect_framework(package_json: &Value, path: &str) -> (String, String) {
    if package_json["dependencies"].get("next").is_some() {
        let path_to_dev;
        if cfg!(target_os = "windows") {
            path_to_dev = format!("\"{}\\node_modules\\next\\dist\\bin\\next\" dev", path)
        }
        else {
            path_to_dev = format!("\"{}/node_modules/next/dist/bin/next\" dev", path)
        }
        ("Next.js".to_string(), format!("node {}", path_to_dev).to_string())
    } else if package_json["dependencies"].get("react").is_some() {
        let path_to_dev;
        if cfg!(target_os = "windows") {
            path_to_dev = format!("\"{}\\node_modules\\react-scripts\\scripts\\start.js\"", path)
        }
        else {
            path_to_dev = format!("\"{}/node_modules/react-scripts/scripts/start.js\"", path)
        }
        ("React".to_string(), format!("node {}", path_to_dev).to_string())
    }  else if  package_json["dependencies"].get("nuxt").is_some(){
        let path_to_dev;
        if cfg!(target_os = "windows") {
            path_to_dev = format!("\"{}\\node_modules\\nuxt\\bin\\nuxt.mjs\" dev", path)

        }
        else {
            path_to_dev = format!("\"{}/node_modules/nuxt/bin/nuxt.mjs\" dev", path)

        }
        ("Nuxt.js".to_string(), format!("node {}", path_to_dev).to_string())
    }    
     else if package_json["dependencies"].get("vue").is_some() {
        let path_to_dev;
        if cfg!(target_os = "windows") {
            path_to_dev = format!("\"{}\\node_modules\\vite\\dist\\node\\cli.js\"", path)
        }
        else {
            path_to_dev = format!("\"{}/node_modules/vite/dist/node/cli.js\"", path)
        }
        ("Vue.js".to_string(), format!("node {}", path_to_dev).to_string())
    }
     else {
        ("Unknown".to_string(), "".to_string())
    }
}

pub fn detect_runtime_version(path: &str) -> String {
    if Path::new(path).join("pnpm-lock.yaml").exists() {
        "pnpm".to_string()
    } else if Path::new(path).join("yarn.lock").exists() {
        "yarn".to_string()
    } else {
        "npm".to_string()
    }
}

pub fn extract_packages(package_json: &Value) -> Vec<Package> {
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
pub fn detect_runtime(project_path: String) -> Result<String, String> {
    if fs::metadata(format!("{}/pnpm-lock.yaml", project_path)).is_ok() {
        Ok("pnpm".to_string())
    } else if fs::metadata(format!("{}/yarn.lock", project_path)).is_ok() {
        Ok("yarn".to_string())
    } else if fs::metadata(format!("{}/package-lock.json", project_path)).is_ok() {
        Ok("npm".to_string())
    } else {
        Err("No supported package manager lock file found".to_string())
    }
}

#[command]
pub fn launch_ide(project_path: String, ide: String) -> Result<(), String> {
    let os = env::consts::OS;

    match os {
        "windows" => {
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd")
                    .args(&["/C", &ide, &project_path])
                    .creation_flags(0x08000000) // CREATE_NO_WINDOW
                    .spawn()
                    .map_err(|e| format!("Failed to launch {}: {}", ide, e))?;
            }       
            
        },
        "macos" | "linux" => {
            println!("Launching IDE: {}", ide);
            let mut command = Command::new(&ide);
            
            // Add appropriate flags based on the IDE
            
            
            command.arg(&project_path)
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
pub fn open_file_explorer(project_path: String) -> Result<(), String> {
    let os = env::consts::OS;
    
    match os {
        "windows" => {
            #[cfg(target_os = "windows")]
            {
                Command::new("explorer")
                    .arg(&project_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                    .spawn()
                    .map_err(|e| format!("Failed to open file explorer: {}", e))?;
            }
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

#[command]
pub fn start_project_creation(window: tauri::Window, runtime: String, framework: String, project_name: String, location: String) -> Result<(), String> {
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
                vec!["pnpm", "dlx", "create-next-app", &create_path, "--ts", "--eslint", "--tailwind", "--src-dir", "--app", "--no-import-alias"]
            }
            ("npm", "next.js") => {
                vec!["npx", "create-next-app@latest", &create_path, "--yes","--ts", "--eslint", "--tailwind", "--src-dir", "--app", "--no-import-alias"]
            }
            ("yarn", "next.js") => {
                vec!["yarn", "create","next-app", &create_path, "--ts", "--eslint", "--tailwind", "--src-dir", "--app", "--no-import-alias"]
            }
            ("pnpm", "vue") => {
                vec!["pnpm", "create", "vue@latest", &project_name, "--typescript", "--eslint-with-prettier"]
            }
            ("npm", "vue") => {
                vec!["npm", "create", "vue@latest", &project_name,"--typescript", "--eslint-with-prettier"]
            }
            ("yarn", "vue") => {
                vec!["yarn", "dlx", "create-vue@latest", &project_name,"--typescript", "--eslint-with-prettier"]
            }
            ("pnpm", "nuxt") => {
                vec![ "pnpm", "dlx","nuxi@latest", "init",&project_name, "--gitInit","--packageManager","pnpm"]
            }
            ("npm", "nuxt") => {
                vec![ "npx", "nuxi@latest", "init", &project_name, "--gitInit","--packageManager","npm"]
            }
            ("yarn", "nuxt") => {
                vec!["yarn", "dlx", "nuxi@latest", "init", &project_name, "--gitInit","--packageManager","yarn"]
            }
            _ => {
                window.emit("creation_status", "Error: Unsupported runtime or frameworks").unwrap();
                return;
            }
        };
        let cmd2 = match (runtime.as_str(), framework.as_str()) {
            ("pnpm", "next.js") => {
                format!("pnpm dlx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", project_name)
            }
            ("npm", "next.js") => {
                format!("npx create-next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", project_name)
            }
            ("yarn", "next.js") => {
                format!("yarn create next-app {} --ts --eslint --tailwind --src-dir --app --no-import-alias", project_name)
            }
            ("pnpm", "vue") => {
                format!("pnpm create vue@latest {} -- --typescript --eslint-with-prettier", project_name)
            }
            ("npm", "vue") => {
                format!("npm create vue@latest {} -- --typescript --eslint-with-prettier", project_name)
            }
            ("yarn", "vue") => {
                format!("yarn dlx create-vue@latest {} -- --typescript --eslint-with-prettier", project_name)
            }
            ("pnpm", "nuxt") => {
                format!("pnpm dlx nuxi@latest init {}  --gitInit --packageManager pnpm", project_name)
            }
            ("npm", "nuxt") => {
                format!("npx nuxi@latest init {}  --gitInit --packageManager npm", project_name)
            }
            ("yarn", "nuxt") => {
                format!("yarn dlx nuxi@latest init {}  --gitInit --packageManager yarn", project_name)
            }
            _ => {
                window.emit("creation_status", "Error: Unsupported runtime or framework").unwrap();
                return;
            }
        };
        println!("Command: {:?}", cmd);
        let output;
        if cfg!(target_os = "windows") {
            println!("Command: {:?}", cmd);
            output = execute_command(&cmd, &location);
        }
        else {
            println!("Command2: {:?}", cmd2);
            output = execute_command(&[&cmd2], &location);
        }
        
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    if framework == "next.js" || framework == "nuxt" {
                        println!("Project created successfully!");
                        window.emit("creation_status", "Project created successfully!").unwrap();
                    }
                    else
                    if framework == "vue" {
                        let install_cmd = format!("{} i", runtime);
                        let install_output = execute_command(&[&install_cmd], &create_path);
                        match install_output {
                            Ok(install_output) => {
                                if install_output.status.success() {
                                    println!("Dependencies installed successfully!");
                                    window.emit("creation_status", "Project created successfully!").unwrap();
                                } else {
                                    let error_message = String::from_utf8_lossy(&install_output.stderr);
                                    println!("Error installing dependencies: {}", error_message);
                                    window.emit("creation_status", format!("Error installing dependencies: {}", error_message)).unwrap();
                                }
                            },
                            Err(e) => {
                                window.emit("creation_status", format!("Failed to execute install command: {}", e)).unwrap();
                            }
                        }
                    }
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    println!("Error: {:?}", output.stderr);
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

#[command]
pub fn delete_site(project_path: String) -> Result<(), String> {
    let path = Path::new(&project_path);
    if path.exists() {
        fs::remove_dir_all(path).map_err(|e| format!("Failed to delete site: {}", e))?;
        Ok(())
    } else {
        Err("Project path does not exist".to_string())
    }
}

#[command]
pub fn start_project(
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

    let (executable, args) = parse_command(&command)?;

    println!("Command: {}", command);
    // Spawn the child process
    let mut command = Command::new(executable);
        command.args(&args)
            .current_dir(&project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000);
        }

let mut child = command.spawn().map_err(|e| e.to_string())?;

    let pid = child.id();

    // Stream output
    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;
    
    println!("stdout: {:?}", stdout);
    println!("stderr: {:?}", stderr);
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
pub fn close_project(state: State<'_, ProjectManager>, pid: u32) -> Result<(), String> {
    let mut projects = state.0.lock().unwrap();
    if projects.remove(&pid).is_some() {
        terminate_process(pid)?;
    }
    Ok(())
}

//create local projects folder incase of new root folder name
#[command]
pub fn create_local_projects_folder(folder_name: Option<String>, path: Option<String>) -> Result<String, String> {
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

//update project path currently not in use
#[command]
pub fn update_project_path(new_path: String) -> Result<String, String> {
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

