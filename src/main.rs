#![allow(dead_code)]
use std::{
    env,
    sync::{Arc, RwLock},
};

mod file;
mod lexer;
mod logger;
mod repl;
mod token;
mod util;

use logger::{log, Logger};

thread_local! {
    static LOG: RwLock<Arc<Logger>> = RwLock::new(Arc::new(Logger {
        prefix: String::from("ti"),
        loglevel: logger::PRINT,
    }));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let a: util::Arguments = util::parse_arguments(&args);

    LOG.with(|l| {
        *l.write().unwrap() = Arc::new(Logger {
            loglevel: a.loglevel,
            prefix: "ti".to_string(),
        })
    });

    log().debug(&format!("process arguments: {:?}", args));
    log().debug(&format!("parsed arguments: {:?}", a));

    util::load_dot_env();

    if a.mode == util::RunMode::Repl {
        repl::repl();
    } else {
        let input_file = match a.in_file_path {
            Some(p) => p,
            None => {
                log().error("no input file specified");
                return;
            }
        };
        let output_file = match a.out_file_path {
            Some(p) => p,
            None => {
                log().error("no output file specified");
                return;
            }
        };
        file::file(&input_file, &output_file);
    }
}
