use clap::{App, Arg};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn read_json_from_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let ser = serde_json::from_reader(reader)?;
    Ok(ser)
}

fn main() {
    let matches = App::new("JSON Editor")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Command-line JSON processing tool")
        .arg(Arg::new("FILE").about("input file"))
        .get_matches();
    if let Some(i) = matches.value_of("FILE") {
        match read_json_from_file(&i) {
            Ok(ser) => println!("{:#?}", ser),
            Err(err) => println!("{}", err),
        };
    }
}
