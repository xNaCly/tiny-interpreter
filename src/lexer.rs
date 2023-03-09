use crate::{
    logger::{self, log},
    token::{Token, TokenKind},
};
use std::time::Instant;

/// Lexer struct that holds the input string, current lexer position and character
pub struct Lexer {
    input: String,
    cur_char: char,
    pos: usize,
    line: usize,
}

impl Lexer {
    /// instantiate a new Lexer
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            cur_char: '\0',
            pos: 0,
            line: 0,
        }
    }

    fn error(&self, msg: &str, description: &str, lexeme: &str) {
        log().syntax_error(&format!(
            "{} at pos '{}' of '{}' on line {}:\n",
            msg, self.pos, self.input, self.line
        ));

        let line = if self.line < 9 {
            format!("0{}", self.line)
        } else {
            format!("{}", self.line)
        };

        let up_arrows = if lexeme.len() > 1 {
            format!(" {}", "^".repeat(lexeme.len()))
        } else {
            "^".to_string()
        };

        println!(
            "{}{} |{} {}\n{}{} {}{}\n\n{}\n",
            logger::COLOR_BLUE,
            line,
            logger::COLOR_RESET,
            self.input,
            logger::COLOR_RED,
            " ".repeat((self.pos - lexeme.len()) + 5) + &up_arrows,
            msg,
            logger::COLOR_RESET,
            description
        );
    }

    fn at_end(&self) -> bool {
        self.pos >= self.input.len() || self.cur_char == '\0'
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

    fn string(&mut self) -> (TokenKind, String) {
        let mut result = String::new();

        // skip the "
        self.advance();

        while self.cur_char != '"' && !self.at_end() {
            result.push(self.cur_char);
            self.advance();
        }

        if self.at_end() {
            self.error(
                "unterminated string",
                "string literals must be terminated with '\"'",
                &result,
            );
            return (TokenKind::UNKNOWN(), result);
        }

        (TokenKind::STRING(result.clone()), result)
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
        let start_time = Instant::now();
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
                ' ' | '\t' | '\r' => {
                    self.advance();
                    continue;
                }
                '\n' => {
                    self.line += 1;
                    self.pos = 0;
                    continue;
                }
                '\0' => token_kind = TokenKind::EOF,
                '"' => {
                    let (t_kind, t_literal) = self.string();
                    token_kind = t_kind;
                    literal = t_literal;
                }
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
                        self.error(
                            &format!("unexpected character '{}'", self.cur_char),
                            "expected a valid character, such as a number, operator, or parenthesis",
                            &literal,
                        );
                        return vec![];
                    }
                }
            }

            let pos = match self.pos {
                0 => 0,
                _ => self.pos - literal.len(),
            };

            let t = Token {
                pos,
                kind: token_kind,
                literal,
            };

            vectors.push(t);

            if !cur_char_num {
                self.advance();
            }
        }

        log().debug(&format!(
            "parsed: {} token{}, took {}µs",
            vectors.len(),
            if vectors.len() == 1 { "" } else { "s" },
            start_time.elapsed().as_micros()
        ));
        vectors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_symbols() {
        let mut lexer = Lexer::new(
            "5192 * 17.9 % + - / ** (  ) ! != = . < > <= >= \"this is a string\"".to_string(),
        );
        let tokens = lexer.lex();
        assert_eq!(tokens[0].kind, TokenKind::INTEGER(5192));
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
        assert_eq!(
            tokens[18].kind,
            TokenKind::STRING("this is a string".to_string())
        );
    }

    #[test]
    fn test_lexer_skip_whitespace() {
        let mut lexer = Lexer::new(" \n\t".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_lexer_skip_unknown() {
        let mut lexer = Lexer::new("unknown".to_string());
        let tokens = lexer.lex();
        for token in tokens {
            assert_eq!(token.kind, TokenKind::UNKNOWN());
        }
    }

    #[test]
    fn test_lexer_string() {
        let mut lexer =
            Lexer::new("\"this is the first string\" \"second string hmmmmm\"".to_string());
        let tokens = lexer.lex();
        assert_eq!(
            tokens[0].kind,
            TokenKind::STRING("this is the first string".to_string())
        );
        assert_eq!(
            tokens[1].kind,
            TokenKind::STRING("second string hmmmmm".to_string())
        );
    }
}
