use std::{sync::Arc, time::SystemTime};
pub const NONE: u8 = 0;
pub const WARN: u8 = 1;
pub const PRINT: u8 = 2;
pub const DEBUG: u8 = 3;

#[derive(Debug)]
pub struct Logger {
    pub loglevel: u8,
}

impl Logger {
    fn format(&self, l_type: &str, l_txt: &str) -> String {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("[{}] {}: {}", now, l_type, l_txt)
    }

    pub fn debug(&self, txt: &str) {
        if self.loglevel >= DEBUG {
            println!("{}", self.format("debug", txt));
        }
    }

    pub fn print(&self, txt: &str) {
        if self.loglevel >= PRINT {
            println!("{}", self.format("print", txt));
        }
    }

    pub fn warn(&self, txt: &str) {
        if self.loglevel >= WARN {
            println!("{}", self.format("warn", txt));
        }
    }
}

/// wrapper for locking and unwrapping the LOG arc
pub fn log<'a>() -> Arc<Logger> {
    return crate::LOG.with(|l| l.read().unwrap().clone());
}
