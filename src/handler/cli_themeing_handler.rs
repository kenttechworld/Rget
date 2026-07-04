use owo_colors::{OwoColorize};
use crate::styling::cli_themeing;

// themeing handler
pub struct CliStyler {
    theme: cli_themeing::CliTheme,
}

impl CliStyler {
    pub fn new(theme: cli_themeing::CliTheme) -> Self {
        Self { theme }
    }

    pub fn header(&self, title: &str) {
        println!("{}", title.style(self.theme.header));
    }

    pub fn normal_text(&self, text: &str) {
        println!("{}", text.style(self.theme.normal_text));
    }

    pub fn success(&self, msg: &str) {
        println!("{}", msg.style(self.theme.success));
    }

    pub fn error(&self, error_msg: &str) {
        println!("{}", error_msg.style(self.theme.error));
    }
}