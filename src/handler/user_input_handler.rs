use crate::handler::run_os_command;

use std::{io::{self, Write}};
use inquire::Confirm;

pub fn keep_terminal_open(){
    // --- KEEP WINDOW OPEN CODE --- 
    print!("\nPress Enter to exit...");
    // Flush stdout to guarantee the text prints before we wait for input
    io::stdout().flush().unwrap(); 
    
    // Read a line into a dummy string, effectively pausing the app
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}

pub fn yes_no_promt(){
    // interactive "Yes/No" prompt
    let ans = Confirm::new("Do you want to upgrade?")
        .with_default(false) // If they just press Enter, it defaults to No
        .with_help_message("Admin promts will popup")
        .prompt();

    match ans {
        Ok(true) => run_os_command::winget_upgrade_installed_programs(), // run winget upgrade --aa forced
        Ok(false) => println!("Exiting.. Have a good day 👍"),
        Err(_) => println!("There was an error parsing your choice or the prompt was interrupted."),
    }
}