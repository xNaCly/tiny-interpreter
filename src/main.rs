#![allow(dead_code)]
use std::{io::{self, Write}, time::Instant};
use token::{Token, TOKENS};
use util::{Logger, ASCII_ART};
use crate::eval::eval_artithmetic;

// use crate::eval::eval_tokens;

mod util;
mod scanner;
mod token;
mod eval;

fn main() {
    let root_start = Instant::now();
    let mut logger = Logger::new();
    println!("{}", ASCII_ART);
    println!("\ttiny interpreter (v0.1.0) - written by xnacly\n");
    loop {
        print!(">>> ");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let loop_start = Instant::now();
        let input = input.trim().replace(" ", "");

        let tokens: Vec<Token> = scanner::scan(input.trim());
        let time_to_parse = loop_start.elapsed();

        if tokens.len() == 0 {
            continue;
        }

        match tokens.get(0).unwrap().ttype {
            TOKENS::EXIT => {
                println!("exiting, inside for: {:?}", root_start.elapsed());
                break;
            }
            TOKENS::UNKNOWN => {
                let error_token = tokens.get(0).unwrap();
                logger.warn(&format!("Could not parse char '{}' at index '{}' of input '{}'", error_token.tparsed_from, error_token.tstart_index + 1, input));
                continue;
            }
            TOKENS::UNKNOWNCMD => {
                logger.warn(&format!("Command '{}' is unknown, see '.help' for a list of commands", input));
                continue;
            }
            TOKENS::DEBUGCMD => {
                logger.print_debug = !logger.print_debug;
                logger.info(&format!("toggling debugging mode to {}", logger.print_debug));
                continue;
            }
            _ => ()
        }

        if logger.print_debug {
            for t in &tokens {
                logger.debug(&format!("{:?}", t));
            }
        }

        println!("{}", eval_artithmetic(tokens));

        logger.debug(&format!("parsing input took: {}ms, evaluating input took: {}ms", time_to_parse.as_millis(), loop_start.elapsed().as_millis()));
    }
}
