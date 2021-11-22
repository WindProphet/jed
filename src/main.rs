use clap::{App, Arg};
use crossterm::style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor};
use crossterm::{queue, QueueableCommand};
use serde_json::{Number, Value};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn read_json_from_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let ser = serde_json::from_reader(reader)?;
    Ok(ser)
}

struct DisplayConfig<'a> {
    w: &'a mut dyn std::io::Write,
    indent: usize,
}

impl<'a> DisplayConfig<'a> {
    fn print_string(&mut self, val: &str) -> Result<(), crossterm::ErrorKind> {
        queue!(
            self.w,
            SetForegroundColor(Color::DarkGreen),
            Print("\""),
            Print(val),
            Print("\""),
            ResetColor
        )?;
        Ok(())
    }

    fn print_null(&mut self) -> Result<(), crossterm::ErrorKind> {
        queue!(
            self.w,
            SetAttribute(Attribute::Bold),
            Print("null"),
            SetAttribute(Attribute::Reset)
        )?;
        Ok(())
    }

    fn print_number(&mut self, val: &Number) -> Result<(), crossterm::ErrorKind> {
        queue!(
            self.w,
            SetForegroundColor(Color::DarkYellow),
            Print(format!("{}", val)),
            ResetColor,
        )?;
        Ok(())
    }

    fn print_bool(&mut self, val: &bool) -> Result<(), crossterm::ErrorKind> {
        queue!(
            self.w,
            SetForegroundColor(Color::DarkMagenta),
            Print(format!("{}", val)),
            ResetColor
        )?;
        Ok(())
    }

    fn print_json(&mut self, val: &Value) -> Result<(), crossterm::ErrorKind> {
        match val {
            Value::Null => self.print_null()?,
            Value::Bool(b) => self.print_bool(b)?,
            Value::String(s) => self.print_string(s)?,
            Value::Number(n) => self.print_number(n)?,
            Value::Array(a) => {
                let mut len = a.len();
                if len == 0 {
                    self.w.queue(Print("[]"))?;
                } else {
                    self.w.queue(Print("["))?.queue(Print("\n"))?;
                    self.indent += 2;
                    for el in a {
                        self.w.queue(Print(" ".repeat(self.indent)))?;
                        self.print_json(el)?;
                        len -= 1;
                        if len != 0 {
                            self.w.queue(Print(","))?;
                        }
                        self.w.queue(Print("\n"))?;
                    }
                    self.indent -= 2;
                    self.w
                        .queue(Print(" ".repeat(self.indent)))?
                        .queue(Print("]"))?;
                }
            }
            Value::Object(o) => {
                let mut len = o.len();
                if len == 0 {
                    self.w.queue(Print("{}"))?;
                } else {
                    self.w.queue(Print("{"))?.queue(Print("\n"))?;
                    self.indent += 2;
                    for (key, el) in o {
                        self.w.queue(Print(" ".repeat(self.indent)))?;
                        self.print_string(key)?;
                        self.w.queue(Print(": "))?;
                        self.print_json(el)?;
                        len -= 1;
                        if len != 0 {
                            self.w.queue(Print(","))?;
                        }
                        self.w.queue(Print("\n"))?;
                    }
                    self.indent -= 2;
                    self.w
                        .queue(Print(" ".repeat(self.indent)))?
                        .queue(Print("}"))?;
                }
            }
        }
        Ok(())
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
}
