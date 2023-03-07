use crate::{
    logger::log,
    token::{Token, TokenKind},
};

pub struct Lexer {
    input: String,
    pub cur_char: char,
    pub pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.clone(),
            cur_char: input
                .chars()
                .nth(0)
                .expect("error while retrieving character from input"),
            pos: 0,
        }
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut vectors = vec![];
        for i in 0..self.input.len() {
            let c = self
                .input
                .chars()
                .nth(i)
                .expect("error while retrieving character from input");

            let token_kind: TokenKind;
            match c {
                '+' => token_kind = TokenKind::PLUS(),
                '-' => token_kind = TokenKind::MINUS(),
                '/' => token_kind = TokenKind::SLASH(),
                '*' => token_kind = TokenKind::ASTERISK(),
                '%' => token_kind = TokenKind::MOD(),
                _ => {
                    if c.is_digit(10) {
                        // TODO: iterate further if next char is also integer
                        let c_as_int = c.to_digit(10).expect(&format!(
                            "error while parsing '{}' at pos:'{}' of '{}'",
                            c, i, self.input
                        )) as usize;
                        token_kind = TokenKind::INTEGER(c_as_int)
                    } else {
                        log().warn(&format!(
                            "Token '{}' at pos:'{}' of '{}' unknown!",
                            c, i, self.input
                        ));
                        // TODO: dont reset lexer output on unknown token
                        return vec![];
                    }
                }
            }
            vectors.push(Token {
                kind: token_kind,
                pos: i,
                literal: String::from(c),
            })
        }
        vectors
    }
}
