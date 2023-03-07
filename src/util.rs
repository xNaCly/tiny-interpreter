use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::logger::log;

/// struct holding cli arguments
pub struct Arguments {
    pub loglevel: u8,
}

const DEFAULT_ARGUMENTS: Arguments = Arguments { loglevel: 1 };

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

pub fn parse_arguments(args: &Vec<String>) -> Arguments {
    if args.len() == 1 {
        return DEFAULT_ARGUMENTS;
    }

    let mut arguments = Arguments { loglevel: 1 };
    let flag_arg = args.get(1).unwrap().as_str();

    if flag_arg.starts_with("-L") {
        let formatted_arg = flag_arg.replace("-L", "");
        match formatted_arg.as_str() {
            "none" => arguments.loglevel = crate::logger::NONE,
            "debug" => arguments.loglevel = crate::logger::DEBUG,
            a => {
                let a_as_int: u8 = match a.parse::<u8>() {
                    Ok(a) => a,
                    Err(b) => {
                        log().warn(&format!("value '{}' not supported for -L: {}", a, b));
                        return arguments;
                    }
                };
                if !(crate::logger::NONE <= a_as_int && a_as_int <= crate::logger::DEBUG) {
                    log().warn(&format!("value '{}' not supported for -L", a));
                }
                arguments.loglevel = a_as_int
            }
        }
    }

    return arguments;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_arguments() {
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
}
