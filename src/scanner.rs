use crate::token::{Token, TOKENS, POSSIBLETOKENVALUE};

pub fn scan(inp: &str) -> Vec<Token> {
    let mut vec_arr = vec![];
    // this parses commands and symbols
    for (i, c) in inp.chars().enumerate() {
        let mut rt = Token {
            tstart_index: i,
            tparsed_from: String::from(c),
            tvalue: POSSIBLETOKENVALUE::NONE,
            ttype: TOKENS::UNKNOWN,
        };
        if inp.starts_with('.'){
            match inp {
                ".exit" => {
                    rt.ttype = TOKENS::EXIT;
                }
                ".debug" => {
                    rt.ttype = TOKENS::DEBUGCMD
                }
                _ => {
                    rt.ttype = TOKENS::UNKNOWNCMD;
                }
            }
        } else {
            // TODO: parse integers
            match c {
                '+' => rt.ttype = TOKENS::PLUS,
                '-' => rt.ttype = TOKENS::MINUS,
                '/' => rt.ttype = TOKENS::SLASH,
                '*' => rt.ttype = TOKENS::ASTERISK,
                '%' => rt.ttype = TOKENS::MODULO,
                _ => {
                    if c.is_numeric() {
                        rt.tvalue = POSSIBLETOKENVALUE::INTEGER(c.to_digit(10).unwrap() as i32);
                        rt.ttype = TOKENS::INTEGER;
                    } else {
                        rt.ttype = TOKENS::UNKNOWN;
                    }
                }
            }
        }
        vec_arr.push(rt);
    }
    return vec_arr;
}
