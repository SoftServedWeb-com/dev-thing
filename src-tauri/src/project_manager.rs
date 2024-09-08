use std::collections::HashMap;
use std::process::Child;
use std::sync::Mutex;
#[cfg(unix)]
use nix::unistd::Pid;
#[cfg(unix)]
use nix::sys::signal::{kill, Signal};
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
#[cfg(target_os = "windows")]
use winapi::um::winnt::{PROCESS_TERMINATE, HANDLE};
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::DWORD;

pub struct ProjectManager(pub Mutex<HashMap<u32, Child>>);

#[cfg(target_os = "windows")]
pub fn terminate_process(pid: u32) -> Result<(), String> {
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
pub fn terminate_process(pid: u32) -> Result<(), String> {
    kill(Pid::from_raw(pid as i32), Signal::SIGTERM).map_err(|e| e.to_string())
}