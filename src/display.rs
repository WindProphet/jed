use crossterm::style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor};
use crossterm::{queue, QueueableCommand};
use serde_json::{Number, Value};

pub struct DisplayConfig<'a> {
    pub w: &'a mut dyn std::io::Write,
    pub indent: usize,
}

impl<'a> DisplayConfig<'a> {
    fn print_string(&mut self, val: &str, is_key: bool) -> Result<(), crossterm::ErrorKind> {
        queue!(
            self.w,
            SetForegroundColor(if is_key {
                Color::DarkRed
            } else {
                Color::DarkGreen
            }),
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

    pub fn print_json(&mut self, val: &Value) -> Result<(), crossterm::ErrorKind> {
        match val {
            Value::Null => self.print_null()?,
            Value::Bool(b) => self.print_bool(b)?,
            Value::String(s) => self.print_string(s, false)?,
            Value::Number(n) => self.print_number(n)?,
            Value::Array(a) => {
                let mut len = a.len();
                if len == 0 {
                    self.w.queue(Print("[]"))?;
                } else {
                    self.w.queue(Print("["))?.queue(Print("\r\n"))?;
                    self.indent += 2;
                    for el in a {
                        self.w.queue(Print(" ".repeat(self.indent)))?;
                        self.print_json(el)?;
                        len -= 1;
                        if len != 0 {
                            self.w.queue(Print(","))?;
                        }
                        self.w.queue(Print("\r\n"))?;
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
                    self.w.queue(Print("{"))?.queue(Print("\r\n"))?;
                    self.indent += 2;
                    for (key, el) in o {
                        self.w.queue(Print(" ".repeat(self.indent)))?;
                        self.print_string(key, true)?;
                        self.w.queue(Print(": "))?;
                        self.print_json(el)?;
                        len -= 1;
                        if len != 0 {
                            self.w.queue(Print(","))?;
                        }
                        self.w.queue(Print("\r\n"))?;
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
