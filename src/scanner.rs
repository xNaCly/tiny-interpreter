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
                '\n' | '\r' | '\t' | ' ' => {
                    continue;
                }
                '+' => rt.ttype = TOKENS::PLUS,
                '-' => rt.ttype = TOKENS::MINUS,
                '/' => rt.ttype = TOKENS::SLASH,
                '*' => rt.ttype = TOKENS::ASTERISK,
                '%' => rt.ttype = TOKENS::MODULO,
                _ => {
                    if c.is_numeric() {
                        rt.ttype = TOKENS::INTEGER;
                        rt.tvalue = POSSIBLETOKENVALUE::INTEGER(c.to_digit(10).expect("Couldn't parse integer") as i32)
                    } else {
                        rt.ttype = TOKENS::UNKNOWN;
                        return vec![rt];
                    }
                }
            }
        }
        vec_arr.push(rt);
    }
    return vec_arr;
}

#[cfg(test)]
mod scan_tests {
    use super::*;

    #[test]
    fn test_scan_integers_separated_by_plus() {
        let inp = "1 + 2 + 3";
        let res = scan(inp);
        assert_eq!(res[0].ttype, TOKENS::INTEGER);
        assert_eq!(res[1].ttype, TOKENS::PLUS);
        assert_eq!(res[2].ttype, TOKENS::INTEGER);
        assert_eq!(res[3].ttype, TOKENS::PLUS);
        assert_eq!(res[4].ttype, TOKENS::INTEGER);
    }

    #[test]
    fn test_scan_integers_separated_by_minus() {
        let inp = "1 - 2 - 3";
        let res = scan(inp);
        assert_eq!(res[0].ttype, TOKENS::INTEGER);
        assert_eq!(res[1].ttype, TOKENS::MINUS);
        assert_eq!(res[2].ttype, TOKENS::INTEGER);
        assert_eq!(res[3].ttype, TOKENS::MINUS);
        assert_eq!(res[4].ttype, TOKENS::INTEGER);
    }

    #[test]
    fn test_scan_integers_separated_by_asterik(){
        let inp = "1 * 2 * 3";
        let res = scan(inp);
        assert_eq!(res[0].ttype, TOKENS::INTEGER);
        assert_eq!(res[1].ttype, TOKENS::ASTERISK);
        assert_eq!(res[2].ttype, TOKENS::INTEGER);
        assert_eq!(res[3].ttype, TOKENS::ASTERISK);
        assert_eq!(res[4].ttype, TOKENS::INTEGER);
    }

    #[test]
    fn test_scan_integers_separated_by_slashes(){
        let inp = "1 / 2 / 3";
        let res = scan(inp);
        assert_eq!(res[0].ttype, TOKENS::INTEGER);
        assert_eq!(res[1].ttype, TOKENS::SLASH);
        assert_eq!(res[2].ttype, TOKENS::INTEGER);
        assert_eq!(res[3].ttype, TOKENS::SLASH);
        assert_eq!(res[4].ttype, TOKENS::INTEGER);
    }

    #[test]
    fn test_scan_integers_separated_by_modulo(){
        let inp = "1 % 2 % 3";
        let res = scan(inp);
        assert_eq!(res[0].ttype, TOKENS::INTEGER);
        assert_eq!(res[1].ttype, TOKENS::MODULO);
        assert_eq!(res[2].ttype, TOKENS::INTEGER);
        assert_eq!(res[3].ttype, TOKENS::MODULO);
        assert_eq!(res[4].ttype, TOKENS::INTEGER);
    }

    #[test]
    fn test_unknown_token(){
        let inp = "ü";
        assert_eq!(scan(inp)[0].ttype, TOKENS::UNKNOWN);
    }

    #[test]
    fn test_unknown_token_between_valid_token(){
        let inp = "1 ü 2 % 3";
        assert_eq!(scan(inp)[0].ttype, TOKENS::UNKNOWN);
    }
}
