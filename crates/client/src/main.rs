mod app;
mod client_handler;
mod db;
mod logger;
mod message_data;
mod notifications;
mod ui;

use app::App;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use log::LevelFilter;
use ratatui::DefaultTerminal;
use std::io;
use std::process::ExitCode;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> ExitCode {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        default_panic(info);
    }));
    let (sender, receiver) = mpsc::unbounded_channel();
    if let Err(_) = logger::init(LevelFilter::Debug, sender) {
        eprintln!("Failed to setup logger!");
        return ExitCode::FAILURE;
    }
    let mut app = match App::new() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let terminal = init_terminal();
    if let Err(e) = ui::run_ui(terminal, &mut app, receiver).await {
        eprintln!("Error occurred running application: {}", e);
        restore_terminal();
        return ExitCode::FAILURE;
    }
    restore_terminal();
    ExitCode::SUCCESS
}

fn init_terminal() -> DefaultTerminal {
    let mut stdout = io::stdout();
    let terminal = ratatui::init();
    let _ = execute!(stdout, EnableMouseCapture);
    terminal
}

fn restore_terminal() {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, DisableMouseCapture);
    ratatui::restore();
}
