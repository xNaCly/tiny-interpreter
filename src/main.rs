#![allow(dead_code)]
use std::{
    env,
    sync::{Arc, RwLock},
};

mod lexer;
mod logger;
mod util;

use logger::{log, Logger};

thread_local! {
    static LOG: RwLock<Arc<Logger>> = RwLock::new(Arc::new(Logger {
        loglevel: logger::PRINT,
    }));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let a: util::Arguments = util::parse_arguments(&args);

    log().debug(&format!("process arguments: {:?}", args));
    LOG.with(|l| {
        *l.write().unwrap() = Arc::new(Logger {
            loglevel: a.loglevel,
        })
    });

    util::load_dot_env();
}
