use std::{process::exit, sync::Arc};
pub const NONE: u8 = 0;
pub const WARN: u8 = 1;
pub const PRINT: u8 = 2;
pub const DEBUG: u8 = 3;

#[derive(Debug)]
pub struct Logger {
    pub loglevel: u8,
    pub prefix: String,
}

pub const COLOR_RED: &str = "\x1b[91m";
pub const COLOR_YELLOW: &str = "\x1b[93m";
pub const COLOR_BLUE: &str = "\x1b[96m";
pub const COLOR_RESET: &str = "\x1b[0m";

impl Logger {
    fn format(&self, l_type: &str, l_txt: &str) -> String {
        let color = match l_type {
            "warn" => COLOR_YELLOW,
            "debug" => COLOR_BLUE,
            _ => {
                if l_type.contains("error") {
                    COLOR_RED
                } else {
                    COLOR_RESET
                }
            }
        };
        if l_type.len() == 0 {
            format!("{}: {}", self.prefix, l_txt)
        } else {
            format!(
                "{}: {}{}{}: {}",
                self.prefix, color, l_type, "\x1b[0m", l_txt
            )
        }
    }

    pub fn debug(&self, txt: &str) {
        if self.loglevel >= DEBUG {
            println!("{}", self.format("debug", txt));
        }
    }

    pub fn print(&self, txt: &str) {
        if self.loglevel >= PRINT {
            println!("{}", self.format("", txt));
        }
    }

    pub fn warn(&self, txt: &str) {
        if self.loglevel >= WARN {
            println!("{}", self.format("warn", txt));
        }
    }

    pub fn error(&self, txt: &str) {
        println!("{}", self.format("fatal error", txt));
        exit(1);
    }

    pub fn syntax_error(&self, txt: &str) {
        println!("{}", self.format("syntax error", txt));
    }
}

/// wrapper for locking and unwrapping the LOG arc
pub fn log<'a>() -> Arc<Logger> {
    return crate::LOG.with(|l| l.read().unwrap().clone());
}
