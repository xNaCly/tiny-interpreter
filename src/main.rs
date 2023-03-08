#![allow(dead_code)]
use std::{
    env,
    io::{self, Write},
    sync::{Arc, RwLock},
};

mod lexer;
mod logger;
mod token;
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

    println!("\ntiny-interpreter - written by xnacly :^)");
    println!("---");
    println!("Type .exit to exit the interpreter");
    let start_time = std::time::Instant::now();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line from stdin");

        match input.trim() {
            ".exit" => break,
            _ => (),
        }

        let mut lexer = lexer::Lexer::new(input.trim().to_string());
        let lex_output = lexer.lex();
        if lex_output.len() > 0 {
            dbg!(lex_output);
        }
    }
    println!("exit, time in repl: {}s", start_time.elapsed().as_secs());
}
