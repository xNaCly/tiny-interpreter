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

    fn advance(&mut self) {
        println!("{}, {}", self.pos, self.cur_char);
        if self.pos + 1 < self.input.len() {
            self.pos += 1;
            self.cur_char = self
                .input
                .chars()
                .nth(self.pos)
                .expect("error while retrieving character from input");
        } else {
            self.cur_char = '\0';
        }
    }

    pub fn number(&mut self) -> (usize, String) {
        let mut result = String::new();
        while self.cur_char.is_digit(10) {
            result.push(self.cur_char);
            self.advance();
        }
        (
            result.parse::<usize>().expect(&format!(
                "could not parse number '{}' at pos '{}'",
                result, self.pos
            )),
            result,
        )
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut vectors = vec![];
        while self.cur_char != '\0' {
            let token_kind: TokenKind;
            let mut literal: String = String::new();
            let cur_char_num = self.cur_char.is_digit(10);
            match self.cur_char {
                '\0' => {
                    token_kind = TokenKind::EOF;
                }
                '+' => {
                    token_kind = TokenKind::PLUS;
                }
                '-' => {
                    token_kind = TokenKind::MINUS;
                }
                '/' => {
                    token_kind = TokenKind::SLASH;
                }
                '*' => {
                    token_kind = TokenKind::ASTERISK;
                }
                '%' => {
                    token_kind = TokenKind::MOD;
                }
                _ => {
                    if cur_char_num {
                        let (number, new_literal) = self.number();
                        literal = new_literal;
                        token_kind = TokenKind::INTEGER(number);
                    } else {
                        log().warn(&format!(
                            "Token '{}' at pos:'{}' of '{}' unknown!",
                            self.cur_char, self.pos, self.input
                        ));
                        break;
                    }
                }
            }

            let t = Token {
                pos: self.pos,
                kind: token_kind,
                literal,
            };

            vectors.push(t);

            // NOTE: number parsing advances on its own, if we didnt match a number we advance here
            if !cur_char_num {
                self.advance();
            }
        }

        vectors
    }
}
