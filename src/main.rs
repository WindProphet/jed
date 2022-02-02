use crate::display::DisplayConfig;
use clap::{App, Arg};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown,
    ScrollUp,
};
use crossterm::ExecutableCommand;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod display;

fn read_json_from_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let ser = serde_json::from_reader(reader)?;
    Ok(ser)
}

macro_rules! key {
    (ctrl+$k:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($k),
            modifiers: KeyModifiers::CONTROL,
        })
    };
    ($k:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($k),
            modifiers: KeyModifiers::NONE,
        })
    };
}

fn listen_events() -> crossterm::Result<()> {
    loop {
        match read()? {
            key!(ctrl + 'c') | key!('q') => return Ok(()),
            key!('j') => {
                std::io::stdout().execute(ScrollDown(1))?;
            }
            key!('k') => {
                std::io::stdout().execute(ScrollUp(1))?;
            }
            _ => (),
        }
    }
}

fn main() {
    let matches = App::new("JSON Editor")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Command-line JSON processing tool")
        .arg(Arg::new("FILE").about("input file"))
        .get_matches();
    std::io::stdout().execute(EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    if let Some(i) = matches.value_of("FILE") {
        match read_json_from_file(i) {
            Ok(ser) => {
                let mut d = DisplayConfig {
                    w: &mut std::io::stdout(),
                    indent: 0,
                };
                d.print_json(&ser).unwrap();
            }
            Err(err) => println!("{}", err),
        };
    }
    listen_events().unwrap();
    disable_raw_mode().unwrap();
    std::io::stdout().execute(LeaveAlternateScreen).unwrap();
}
