use crate::{handler::theme_checker_handler, handler::user_input_handler, handler::run_os_command};

use figlet_rs::FIGlet;

pub fn show_tui(){
    tui_header();
    let cli_themer = theme_checker_handler::theme_checker();

    cli_themer.normal_text("\n👇 Programs ready to be upgraded 👇\n");
    run_os_command::winget_upgrade();

    // take user input handler
    user_input_handler::yes_no_promt();

    // keep window open
    user_input_handler::keep_terminal_open();
    
}

fn tui_header(){
    let cli_themer = theme_checker_handler::theme_checker();
    let slant_font = FIGlet::slant().unwrap();
    let figure = slant_font.convert("RUST-GET");
    
    if let Some(art) = figure {
        let art_string: String = art.to_string();
        cli_themer.header(&art_string);
    }
}