use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use crate::logger::log;

#[derive(Debug, PartialEq)]
pub enum RunMode {
    Repl,
    File,
}

/// struct holding cli arguments
#[derive(Debug)]
pub struct Arguments {
    pub loglevel: u8,
    pub mode: RunMode,
    pub in_file_path: Option<String>,
    pub out_file_path: Option<String>,
}

const DEFAULT_ARGUMENTS: Arguments = Arguments {
    loglevel: 1,
    mode: RunMode::Repl,
    in_file_path: None,
    out_file_path: None,
};

/// Loads the `.env` file located relatively from the current `cwd`.
/// Does nothing if no `.env` file is present. This behavior is useful for switching from dev to prod env.
pub fn load_dot_env() {
    let file_name = ".env";
    log().debug(&format!(
        "trying to load env variables from '{}'",
        file_name
    ));
    let file = match File::open(file_name) {
        Ok(ok) => ok,
        Err(_) => {
            log().warn("error while loading env: file not found, skipping loading env");
            return;
        }
    };

    log().debug("found the following valid env vars:");

    for (_, line) in BufReader::new(file).lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                log().warn("error while loading env: Couldn't read line");
                return;
            }
        };
        if line.contains('=') {
            let split_line: Vec<&str> = line.split('=').collect();
            let (key, value) = (split_line[0], split_line[1]);
            log().debug(&format!("k:\"{}\" | v:\"{}\"", key, value));
            env::set_var(key, value)
        }
    }
    log().debug("set the given env variables")
}

/// Loads the environment variable for `key` and returns its value.
///
/// Panics if `key` could not be found.
///
pub fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(v) => v,
        Err(_) => {
            panic!("env variable '{}' is not set", key);
        }
    }
}

/// parses the cli arguments and returns a struct containing the parsed arguments
pub fn parse_arguments(args: &Vec<String>) -> Arguments {
    if args.len() == 1 {
        return DEFAULT_ARGUMENTS;
    }

    let mut arguments = Arguments {
        loglevel: 1,
        mode: RunMode::Repl,
        out_file_path: None,
        in_file_path: None,
    };

    for (_, arg) in args.iter().skip(1).enumerate() {
        if arg == "--help" || arg == "-h" {
            help(args.get(0).unwrap(), false);
        }
        match arg.get(0..2) {
            Some(value) => match value {
                "-L" => match arg.get(2..).expect("no loglevel given") {
                    "none" => arguments.loglevel = crate::logger::NONE,
                    "debug" => arguments.loglevel = crate::logger::DEBUG,
                    a => {
                        let a_as_int: u8 = match a.parse::<u8>() {
                            Ok(a) => a,
                            Err(b) => {
                                log().warn(&format!(
                                    "value '{}' not supported for -L: {}, reseting to loglevel 1",
                                    a, b
                                ));
                                return arguments;
                            }
                        };
                        if !(crate::logger::NONE <= a_as_int && a_as_int <= crate::logger::DEBUG) {
                            log().warn(&format!(
                                "value '{}' not supported for -L, reseting to loglevel 1",
                                a
                            ));
                        }
                        arguments.loglevel = a_as_int
                    }
                },
                "-o" => {
                    if arg.contains('=') {
                        arguments.out_file_path = Some(
                            arg.split("=")
                                .last()
                                .expect("flag -o given, but no value")
                                .to_string(),
                        );
                    } else {
                        log().warn("no output file given, switching to repl mode");
                    }
                }
                _ => {
                    if arg.chars().nth(0).expect("no char at index 0") == '-' {
                        log().warn(&format!("unrecognized command-line option '{}'", arg));
                        help(args.get(0).unwrap(), true);
                    } else {
                        arguments.mode = RunMode::File;
                        arguments.in_file_path = Some(arg.to_string());
                    }
                }
            },
            None => {
                return arguments;
            }
        }
    }

    arguments
}

fn help(process_name: &str, short: bool) {
    if short {
        println!("usage: {} [options] file...", process_name);
    } else {
        let help_str = "usage: {} file [options] \
                        \nOptions:\
                        \n\t-L<loglevel>    set the loglevel (0-3) (default: 1)\
                        \n\t\t 0: none\
                        \n\t\t 1: warn\
                        \n\t\t 2: print\
                        \n\t\t 3: debug
                        \n\t-h, --help      display this help and exit\
                        \n\t-o <file>       write output to <file>\
                        \n\nThe input file is the last argument, if it is not a file, the program will run in REPL mode.\
                        \nTo write the output to a file use the -o <file> flag.\
                        ";
        println!("{}", help_str)
    }
    exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_arguments_log_level() {
        let mut args = vec!["filename".to_string(), "-Lnone".to_string()];
        let mut a: Arguments = parse_arguments(&args);
        assert_eq!(a.loglevel, crate::logger::NONE);
        args = vec!["filename".to_string(), "-Ldebug".to_string()];
        a = parse_arguments(&args);
        assert_eq!(a.loglevel, crate::logger::DEBUG);
        args = vec!["filename".to_string(), "-L2".to_string()];
        a = parse_arguments(&args);
        assert_eq!(a.loglevel, crate::logger::PRINT);
    }

    #[test]
    fn test_parse_arguments_files() {
        let mut args = vec!["filename".to_string(), "file1".to_string()];
        let mut a: Arguments = parse_arguments(&args);
        assert_eq!(a.mode, RunMode::File);
        assert_eq!(a.in_file_path, Some("file1".to_string()));
        args = vec![
            "filename".to_string(),
            "file1".to_string(),
            "-o=file2".to_string(),
        ];
        a = parse_arguments(&args);
        assert_eq!(a.mode, RunMode::File);
        assert_eq!(a.in_file_path, Some("file1".to_string()));
        assert_eq!(a.out_file_path, Some("file2".to_string()));
    }

    #[test]
    fn test_parse_arguments_repl() {
        let args = vec!["filename".to_string()];
        let a: Arguments = parse_arguments(&args);
        assert_eq!(a.mode, RunMode::Repl);
    }
}
