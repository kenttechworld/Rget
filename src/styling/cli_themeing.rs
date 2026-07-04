use owo_colors::{Style};

// themeing model
pub struct CliTheme {
    pub success: Style,
    pub error: Style,
    pub header: Style,
    pub normal_text: Style,
}

impl CliTheme {
    pub fn dark() -> Self {
        Self {
            success: Style::new().green().bold(),
            error: Style::new().red().bold().underline(),
            header: Style::new().magenta().bold(),
            normal_text: Style::new().white(),
        }
    }

    pub fn light() -> Self {
        Self {
            success: Style::new().blue().bold(), 
            error: Style::new().red().bold(),
            header: Style::new().cyan().bold(),
            normal_text: Style::new().black()
        }
    }
}