use std::env;

mod tui;
mod core;

use tui::App;

fn main() {
    let args: Vec<String> = env::args().collect();

    let files = &args.as_slice()[1..];

    let terminal = ratatui::init();
    let app_result = App::open(files).run(terminal);
    ratatui::restore();
}

