use crate::handler::theme_checker_handler;
use std::{process::Command};

// run powershell commandvmodel
pub fn power_shell_command(command: &str){
    let cli_themer = theme_checker_handler::theme_checker();

    // Note: Use "pwsh" instead of "powershell" if targeting PowerShell Core 
    let output = Command::new("pwsh")
        .args(["-Command", command])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        cli_themer.success(&stdout);
    } else {
        cli_themer.error("\n!!-PowerShell Eror-!!\nSome programs did not upgrade\nThey may need to be upgraded from within the programs themself!!\n");
    }
}