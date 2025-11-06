mod audio;
mod cli;
mod tui;

use clap::Parser;
use cli::Cli;
use tui::App;

fn main() {
    let args = Cli::parse();
    App::from(args.clone()).run();
}
