mod tui;
mod core;

use tui::App;

fn main() {
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
}

