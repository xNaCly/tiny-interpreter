use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::LOG;

/// Loads the `.env` file located relatively from the current `cwd`.
/// Does nothing if no `.env` file is present. This behavior is useful for switching from dev to prod env.
pub fn load_dot_env() {
    let file_name = ".env";
    LOG.debug(&format!(
        "trying to load env variables from '{}'",
        file_name
    ));
    let file = match File::open(file_name) {
        Ok(ok) => ok,
        Err(_) => {
            LOG.warn("error while loading env: file not found, skipping loading env");
            return;
        }
    };

    LOG.debug("found the following valid env vars:");

    for (_, line) in BufReader::new(file).lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                LOG.warn("error while loading env: Couldn't read line");
                return;
            }
        };
        if line.contains('=') {
            let split_line: Vec<&str> = line.split('=').collect();
            let (key, value) = (split_line[0], split_line[1]);
            LOG.debug(&format!("k:\"{}\" | v:\"{}\"", key, value));
            env::set_var(key, value)
        }
    }
    LOG.debug("set the given env variables")
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
