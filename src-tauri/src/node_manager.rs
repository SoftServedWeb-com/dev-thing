use std::process::{Command, Stdio};
use std::env;
use std::fs;
use std::path::Path;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use tauri::command;

pub fn ensure_node_installed() -> Result<(), String> {
    // Check if Node.js is installed
    if Command::new("node").arg("-v").stdout(Stdio::null()).status().is_ok() {
        return Ok(());
    }

    // Check if nvm is installed
    if !Command::new("nvm").arg("--version").stdout(Stdio::null()).status().is_ok() {
        install_nvm()?;
    }

    // Install Node.js using nvm
    install_node_with_nvm()
}

#[command]
pub fn install_nvm() -> Result<(), String> {
    let os = env::consts::OS;
    let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE"))
        .map_err(|e| format!("Failed to get home directory: {}", e))?;

    match os {
        "windows" => install_nvm_windows(&home_dir),
        "macos" | "linux" => install_nvm_unix(&home_dir),
        _ => Err("Unsupported operating system".to_string()),
    }
}

fn install_nvm_windows(home_dir: &str) -> Result<(), String> {
    let nvm_install_script = "https://raw.githubusercontent.com/coreybutler/nvm-windows/master/nvm-setup.exe";
    let nvm_setup_path = Path::new(home_dir).join("nvm-setup.exe");

    // Download NVM installer
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(&format!("Invoke-WebRequest -Uri {} -OutFile {}", nvm_install_script, nvm_setup_path.display()))
        .output()
        .map_err(|e| format!("Failed to download NVM installer: {}", e))?;

    if !output.status.success() {
        return Err(format!("Failed to download NVM installer: {}", String::from_utf8_lossy(&output.stderr)));
    }

    // Run NVM installer
    let output = Command::new(nvm_setup_path)
        .arg("/SILENT")
        .arg("/NORESTART")
        .output()
        .map_err(|e| format!("Failed to run NVM installer: {}", e))?;

    if !output.status.success() {
        return Err(format!("Failed to install NVM: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

fn install_nvm_unix(home_dir: &str) -> Result<(), String> {
    let nvm_install_script = "https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh";

    let output = Command::new("bash")
        .arg("-c")
        .arg(&format!("curl -o- {} | bash", nvm_install_script))
        .output()
        .map_err(|e| format!("Failed to install NVM: {}", e))?;

    if !output.status.success() {
        return Err(format!("Failed to install NVM: {}", String::from_utf8_lossy(&output.stderr)));
    }

    // Source NVM in the current shell
    let nvm_dir = Path::new(home_dir).join(".nvm");
    std::env::set_var("NVM_DIR", nvm_dir);
    
    let output = Command::new("bash")
        .arg("-c")
        .arg("[ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\"")
        .output()
        .map_err(|e| format!("Failed to source NVM: {}", e))?;

    if !output.status.success() {
        return Err(format!("Failed to source NVM: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

fn install_node_with_nvm() -> Result<(), String> {
    let os = env::consts::OS;
    let command = if os == "windows" {
        Command::new("cmd")
            .args(&["/C", "nvm", "install", "node"])
            .output()
    } else {
        Command::new("bash")
            .arg("-c")
            .arg("source $HOME/.nvm/nvm.sh && nvm install node")
            .output()
    };

    match command {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(format!("Failed to install Node.js: {}", String::from_utf8_lossy(&output.stderr)))
            }
        }
        Err(e) => Err(format!("Failed to execute nvm install node command: {}", e)),
    }
}

#[command]
pub fn check_node_installed() -> Result<bool, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "node -v"])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("node -v")
            .output()
    };

    #[cfg(target_os = "windows")]
    {
        output.creation_flags(0x08000000) // CREATE_NO_WINDOW
    }
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}

fn run_nvm_command(args: &[&str]) -> Result<String, String> {
    let shell = if cfg!(target_os = "windows") { "cmd" } else { "bash" };
    let nvm_init = if cfg!(target_os = "windows") {
        r#"
        @echo off
        call %APPDATA%\nvm\nvm.exe
        "#
    } else {
        r#"
        export NVM_DIR="$HOME/.nvm"
        [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
        "#
    };

    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new(shell);
        cmd.args(&["/C", nvm_init, "&&"]);
        cmd.args(&["nvm"]);
        cmd.args(args);
        cmd
    } else {
        let mut cmd = Command::new(shell);
        cmd.arg("-c");
        let full_command = format!("{}\nnvm {}", nvm_init, args.join(" "));
        cmd.arg(full_command);
        cmd
    };

    let output = command.output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

#[command]
pub fn check_nvm_installed() -> Result<bool, String> {
    match run_nvm_command(&["--version"]) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

