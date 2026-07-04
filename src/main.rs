mod handler;
mod styling;
mod tui;
mod model;

use crate::tui::main_tui;

fn main() {
    main_tui::show_tui();
}