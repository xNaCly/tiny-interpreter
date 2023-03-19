use crate::{lexer::Lexer, logger::log};
use std::{fs::File, io::Write};

pub fn file(file_in: &str, file_out: &str) {
    let start_time = std::time::Instant::now();

    if file_in.is_empty() {
        log().error("no input file specified");
    } else if file_in == file_out {
        log().error("input and output file are the same, ti would override the input file");
    }

    log().debug(&format!("opening file '{}'", file_in));

    let file_content = match std::fs::read_to_string(file_in) {
        Err(e) => {
            log().error(&format!("Unable to read file '{}' due to: {}", file_in, e));
            "".to_string()
        }
        Ok(v) => v,
    };

    log().debug(&format!("done reading file '{}'", file_in));

    log().debug("lexing input file");

    let output = Lexer::new(file_content).lex();

    if output.is_empty() {
        log().error("error, no output generated");
    } else {
        log().debug("starting to write output");
    }

    if file_out.is_empty() {
        log().debug("no out file specified, printing to stdout");
        println!("{:?}", output);
    } else {
        let mut file = match File::create(file_out) {
            Err(e) => {
                log().error(&format!("Unable to read file '{}' due to: {}", file_out, e));
                return
            }
            Ok(v) => v,
        };
        for token in output {
            writeln!(&mut file, "{:?}", token).expect("Unable to write to file");
        }
    }

    log().print(&format!(
        "done, took {}ms",
        start_time.elapsed().as_millis()
    ));
}
