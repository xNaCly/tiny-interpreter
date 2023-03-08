use crate::{
    logger::log,
    token::{Token, TokenKind},
};

pub struct Lexer {
    input: String,
    cur_char: char,
    pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.replace(" ", ""),
            cur_char: '\0',
            pos: 0,
        }
    }

    fn at_end(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn advance(&mut self) {
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

    fn number(&mut self) -> (TokenKind, String) {
        let mut is_float = false;
        let mut result = String::new();
        while self.cur_char.is_digit(10) {
            result.push(self.cur_char);
            self.advance();
        }
        if self.cur_char == '.' && self.peek().is_digit(10) {
            is_float = true;

            result.push(self.cur_char);

            // skip the '.'
            self.advance();

            while self.cur_char.is_digit(10) {
                result.push(self.cur_char);
                self.advance();
            }
        }
        if is_float {
            dbg!(&result);
            (
                TokenKind::FLOAT(result.parse::<f64>().expect("failed to parse float")),
                result,
            )
        } else {
            (
                TokenKind::INTEGER(result.parse::<usize>().expect("failed to parse integer")),
                result,
            )
        }
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.input.chars().nth(self.pos + 1).expect("Couldn't peek")
        }
    }

    fn peek_equals(&self, c: char) -> bool {
        if self.at_end() {
            return false;
        }

        match self.input.chars().nth(self.pos + 1) {
            Some(next_char) => next_char == c,
            None => false,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut vectors = vec![];
        if self.input.is_empty() {
            return vectors;
        }

        self.cur_char = self
            .input
            .chars()
            .nth(self.pos)
            .expect("error while retrieving character from input");

        while self.cur_char != '\0' {
            let token_kind: TokenKind;
            let mut literal: String = String::new();
            let cur_char_num = self.cur_char.is_digit(10);
            match self.cur_char {
                '\0' => token_kind = TokenKind::EOF,
                '+' => token_kind = TokenKind::PLUS,
                '.' => token_kind = TokenKind::DOT,
                '-' => token_kind = TokenKind::MINUS,
                '/' => token_kind = TokenKind::SLASH,
                '(' => token_kind = TokenKind::OPENPAREN,
                ')' => token_kind = TokenKind::CLOSEPAREN,
                '%' => token_kind = TokenKind::MOD,
                '=' => token_kind = TokenKind::EQUAL,
                ':' => {
                    token_kind = if self.peek_equals('=') {
                        self.advance();
                        TokenKind::ASSIGN
                    } else {
                        TokenKind::UNKNOWN()
                    }
                }
                '<' => {
                    token_kind = if self.peek_equals('=') {
                        self.advance();
                        TokenKind::LESSTHANEQUAL
                    } else {
                        TokenKind::LESSTHAN
                    }
                }
                '>' => {
                    token_kind = if self.peek_equals('=') {
                        self.advance();
                        TokenKind::GREATERTHANEQUAL
                    } else {
                        TokenKind::GREATERTHAN
                    }
                }
                '!' => {
                    token_kind = if self.peek_equals('=') {
                        self.advance();
                        TokenKind::BANGEQUAL
                    } else {
                        TokenKind::BANG
                    }
                }
                '*' => {
                    token_kind = if self.peek_equals('*') {
                        self.advance();
                        TokenKind::EXPONENT
                    } else {
                        TokenKind::ASTERISK
                    }
                }
                _ => {
                    if cur_char_num {
                        let (number, new_literal) = self.number();
                        literal = new_literal;
                        token_kind = number;
                    } else {
                        log().warn(&format!(
                            "Token '{}' at pos:'{}' of '{}' unknown!",
                            self.cur_char, self.pos, self.input
                        ));
                        token_kind = TokenKind::UNKNOWN();
                        literal = self.cur_char.to_string();
                    }
                }
            }

            let t = Token {
                pos: self.pos,
                kind: token_kind,
                literal,
            };

            vectors.push(t);

            if !cur_char_num {
                self.advance();
            }
        }

        vectors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_symbols() {
        let mut lexer = Lexer::new("5 * 17.9 % + - / ** (  ) ! != = . < > <= >=".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens[0].kind, TokenKind::INTEGER(5));
        assert_eq!(tokens[1].kind, TokenKind::ASTERISK);
        assert_eq!(tokens[2].kind, TokenKind::FLOAT(17.9));
        assert_eq!(tokens[3].kind, TokenKind::MOD);
        assert_eq!(tokens[4].kind, TokenKind::PLUS);
        assert_eq!(tokens[5].kind, TokenKind::MINUS);
        assert_eq!(tokens[6].kind, TokenKind::SLASH);
        assert_eq!(tokens[7].kind, TokenKind::EXPONENT);
        assert_eq!(tokens[8].kind, TokenKind::OPENPAREN);
        assert_eq!(tokens[9].kind, TokenKind::CLOSEPAREN);
        assert_eq!(tokens[10].kind, TokenKind::BANG);
        assert_eq!(tokens[11].kind, TokenKind::BANGEQUAL);
        assert_eq!(tokens[12].kind, TokenKind::EQUAL);
        assert_eq!(tokens[13].kind, TokenKind::DOT);
        assert_eq!(tokens[14].kind, TokenKind::LESSTHAN);
        assert_eq!(tokens[15].kind, TokenKind::GREATERTHAN);
        assert_eq!(tokens[16].kind, TokenKind::LESSTHANEQUAL);
        assert_eq!(tokens[17].kind, TokenKind::GREATERTHANEQUAL);
    }
}
