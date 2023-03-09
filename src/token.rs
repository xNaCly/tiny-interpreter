#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    EOF,
    UNKNOWN(),

    // single character tokens
    PLUS,
    MINUS,
    COMMA,
    SEMICOLON,
    SLASH,
    ASTERISK,
    DOT,

    // one or two
    MOD,
    OPENPAREN,
    CLOSEPAREN,
    RIGHTBRACE,
    LEFTBRACE,
    BANG,
    BANGEQUAL,
    EQUAL,
    LESSTHAN,
    GREATERTHAN,
    LESSTHANEQUAL,
    GREATERTHANEQUAL,
    EXPONENT,

    // literals
    IDENTIFIER(String),
    INTEGER(usize),
    FLOAT(f64),
    STRING(String),

    // keywords
    AND,
    OR,
    FUN,
    FOR,
    IF,
    ELSE,
    TRUE,
    FALSE,
    NIL,
    RETURN,
    VAR,
    WHILE,
    CLASS,
    SUPER,
    THIS,
    PRINT,
}

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub kind: TokenKind,
    pub literal: String,
    pub line: usize,
}
