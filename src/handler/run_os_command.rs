use crate::model::os_commands;

pub fn winget_upgrade(){
    os_commands::power_shell_command("winget upgrade");
}

pub fn winget_upgrade_installed_programs(){
    os_commands::power_shell_command("winget upgrade --all --accept-package-agreements --accept-source-agreements");
}