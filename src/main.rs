#![allow(dead_code)]
use std::env;
mod logger;
mod util;

const LOG: logger::Logger = logger::Logger {
    loglevel: logger::DEBUG,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let _: util::Arguments = util::parse_arguments(&args);
    LOG.debug(&format!("process arguments: {:?}", args));
    util::load_dot_env();
}
