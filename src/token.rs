#[derive(Debug, PartialEq)]
pub enum TokenKind {
    EOF,
    UNKNOWN(),
    /// [0-9]+
    INTEGER(usize),
    /// [0-9]+
    FLOAT(f64),
    /// :=
    ASSIGN,
    /// +
    PLUS,
    /// -
    MINUS,
    /// /
    SLASH,
    /// *
    ASTERISK,
    /// **
    EXPONENT,
    /// %
    MOD,
    /// (
    OPENPAREN,
    /// )
    CLOSEPAREN,
    /// !
    BANG,
    /// !=
    BANGEQUAL,
    /// .
    DOT,
    /// =
    EQUAL,
    /// <
    LESSTHAN,
    /// >
    GREATERTHAN,
    /// <=
    LESSTHANEQUAL,
    /// >=
    GREATERTHANEQUAL,
}

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub kind: TokenKind,
    pub literal: String,
}
