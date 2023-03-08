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

impl Logger {
    fn format(&self, l_type: &str, l_txt: &str) -> String {
        let color = match l_type {
            "warn" => "\x1b[93m",
            "debug" => "\x1b[96m",
            "fatal error" => "\x1b[91m",
            _ => "\x1b[0m",
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
}

/// wrapper for locking and unwrapping the LOG arc
pub fn log<'a>() -> Arc<Logger> {
    return crate::LOG.with(|l| l.read().unwrap().clone());
}
