use terminal_colorsaurus::{theme_mode, QueryOptions, ThemeMode};

use crate::{handler::cli_themeing_handler, styling::cli_themeing::CliTheme};

// theme checker Handler
pub fn theme_checker() -> cli_themeing_handler::CliStyler{
    match theme_mode(QueryOptions::default()) {
        Ok(ThemeMode::Dark) => {
            //println!("Dark mode ON!");
            return cli_themeing_handler::CliStyler::new(CliTheme::dark())
        }
        Ok(ThemeMode::Light) => {
            //println!("Light Mode ON!");
            return cli_themeing_handler::CliStyler::new(CliTheme::light())
        }
        Err(_) => {
            println!("I failed 😞. Defaulting to Light Theme.");
            return cli_themeing_handler::CliStyler::new(CliTheme::light())
        }
    }
}