use app::App;
use args::Args;
use error::Error;
use termint::{enums::Color, widgets::StrSpanExtension};

mod app;
mod args;
mod board;
mod board_tui;
mod cell;
mod error;

fn main() {
    if let Err(e) = run() {
        println!("{} {e}", "Error:".fg(Color::Red));
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(std::env::args())?;
    if args.help {
        Args::help();
        return Ok(());
    }

    let mut app = App::new(args.size, args.win_len);
    app.run()
}
