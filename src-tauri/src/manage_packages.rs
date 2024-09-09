use tauri::{command, Window};
use std::thread;
use crate::utils::execute_command;
//install, update, delete, reinstall dependencies
#[command]
pub fn install_dependency(
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
        _ => return Err("Unsupported runtime".to_string()),
    };
    println!("Executing command: {}", cmd);
    thread::spawn(move || {
        window.emit("install_status", "Installing dependency...").unwrap();
        let output = execute_command(&[&cmd], &project_path);
       
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
pub fn update_dependency(
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
            _ => return Err("Unsupported runtime".to_string()),
        }
    } else {
        match runtime.as_str() {
            "pnpm" => format!("pnpm update {}", dependency),
            "npm" => format!("npm update {}", dependency),
            "yarn" => format!("yarn upgrade {}", dependency),
            _ => return Err("Unsupported runtime".to_string()),
        }
    };

    println!("Executing command: {}", cmd);
    thread::spawn(move || {
        window.emit("update_status", "Updating dependency...").unwrap();
        let output = execute_command(&[&cmd], &project_path);
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
pub fn delete_dependency(
    window: Window,
    project_path: String,
    runtime: String,
    dependency: String,
) -> Result<(), String> {
    let cmd = match runtime.as_str() {
        "pnpm" => format!("pnpm remove {}", dependency),
        "npm" => format!("npm uninstall {}", dependency),
        "yarn" => format!("yarn remove {}", dependency),
        _ => return Err("Unsupported runtime".to_string()),
    };
    println!("Executing command: {}", cmd);
    thread::spawn(move || {
        window.emit("delete_status", "Deleting dependency...").unwrap();
        let output = execute_command(&[&cmd], &project_path);
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
pub fn reinstall_dependencies(
    window: Window,
    project_path: String,
    runtime: String,
) -> Result<(), String> {
    let cmd = match runtime.as_str() {
        "pnpm" => "pnpm install --force".to_string(),
        "npm" => "npm install --force".to_string(),
        "yarn" => "yarn install --force".to_string(),
        _ => return Err("Unsupported runtime".to_string()),
    };
    println!("Executing command: {}", cmd);
    thread::spawn(move || {
        window.emit("reinstall_status", "Reinstalling dependencies...").unwrap();
        let output = execute_command(&[&cmd], &project_path);
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
