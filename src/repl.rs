use std::io::{self, Write};

use crate::{lexer::Lexer, logger::log};

pub fn repl() {
    log().debug("Starting repl");
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

        let mut lexer = Lexer::new(input.trim().to_string());
        let lex_output = lexer.lex();
        if lex_output.len() > 0 {
            dbg!(lex_output);
        }
    }
    println!("exit, time in repl: {}s", start_time.elapsed().as_secs());
}
