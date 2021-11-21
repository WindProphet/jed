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

fn print_color_json(val: &Value, indent: usize) {
    match val {
        Value::Null => print!("null"),
        Value::Bool(b) => print!("{}", b),
        Value::Number(n) => print!("{}", n),
        Value::String(s) => print!("{}", s),
        Value::Array(a) => {
            print!("[\n");
            for el in a {
                let repeated = " ".repeat(indent + 2);
                print!("{}", repeated);
                print_color_json(el, indent + 2);
                print!(",\n");
            }
            print!("{}]", " ".repeat(indent));
        }
        Value::Object(o) => {
            print!("{{\n");
            for (key, val) in o {
                let repeated = " ".repeat(indent + 2);
                print!("{}{}: ", repeated, key);
                print_color_json(val, indent + 2);
                print!(",\n");
            }
            print!("{}}}", " ".repeat(indent));
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
    if let Some(i) = matches.value_of("FILE") {
        match read_json_from_file(i) {
            Ok(ser) => print_color_json(&ser, 0),
            Err(err) => println!("{}", err),
        };
    }
}
