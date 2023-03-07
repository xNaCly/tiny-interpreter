#[derive(Debug)]
pub enum TokenKind {
    UNKNOWN(),
    /// [0-9]+
    INTEGER(usize),
    /// +
    PLUS(),
    /// -
    MINUS(),
    /// /
    SLASH(),
    /// *
    ASTERISK(),
    /// mod
    MOD(),
}

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub kind: TokenKind,
    pub literal: String,
}
