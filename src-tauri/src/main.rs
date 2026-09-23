#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

const COMPILER_EXE: &str = r"E:\Vasi\hey BUDDY\4454\thigazh-compiler\target\debug\thigazh-compiler.exe";
const COMPILER_WORKDIR: &str = r"E:\Vasi\hey BUDDY\4454\thigazh-compiler";

#[tauri::command]
fn compile(path: String, board: String, flash: bool) -> Result<String, String> {
    if !std::path::Path::new(COMPILER_EXE).exists() {
        return Err(format!("Compiler exe not found at: {}", COMPILER_EXE));
    }
    if !std::path::Path::new(&path).exists() {
        return Err(format!("Source file not found at: {}", path));
    }

    let mut cmd = Command::new(COMPILER_EXE);
    cmd.current_dir(COMPILER_WORKDIR);
    cmd.arg(&path);
    if flash {
        cmd.arg("--flash").arg("--board").arg(&board);
    }
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let output = cmd
        .output()
        .map_err(|e| format!("Compiler run ஆகவில்லை: {}", e))?;

    let mut result = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        result.push_str("\n--- stderr ---\n");
        result.push_str(&stderr);
    }
    result.push_str(if output.status.success() {
        "\n✅ முடிந்தது.\n"
    } else {
        "\n❌ பிழையுடன் முடிந்தது.\n"
    });
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
