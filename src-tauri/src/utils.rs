use std::process::Command;
use shlex::Shlex;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub fn parse_command(command: &str) -> Result<(String, Vec<String>), String> {
    let lexer = Shlex::new(command);
    let mut parts = lexer.collect::<Vec<String>>();
    let executable = parts
        .get(0)
        .ok_or("Failed to parse command")?
        .to_string();
    let args = parts.split_off(1);
    Ok((executable, args))
}

pub fn execute_command(cmd: &[&str], project_path: &str) -> std::io::Result<std::process::Output> {
    println!("Executing command: {:?}", cmd);
    
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd_process = Command::new("cmd");
        cmd_process.arg("/C");
        cmd_process.args(cmd);
        #[cfg(target_os = "windows")]
        cmd_process.creation_flags(0x08000000); // CREATE_NO_WINDOW
        cmd_process
    } else {
        let mut cmd_process = Command::new("sh");
        cmd_process.arg("-c");
        cmd_process.arg(&cmd.join(" "));
        cmd_process
    };

    let cmd_str = cmd.join(" ");
    if cmd_str.contains("npm") {
        command.env("npm_config_user_agent", "npm");
    }

    command.current_dir(project_path).output()
}