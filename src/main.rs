#![allow(dead_code)]
mod logger;
mod util;

const LOG: logger::Logger = logger::Logger {
    loglevel: logger::DEBUG,
};

// TODO: enable debug mode via cli
fn main() {
    util::load_dot_env();
}
