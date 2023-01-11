#[derive(Debug, PartialEq, Eq)]
pub enum TOKENS {
    ERROR,
    EXIT,
    UNKNOWN,
    UNKNOWNCMD,
    DEBUGCMD,
    INTEGER,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    MODULO
}

#[derive(Debug)]
pub enum POSSIBLETOKENVALUE {
    INTEGER(i32), 
    NONE,
}

#[derive(Debug)]
pub struct Token {
    pub tstart_index: usize,
    pub tparsed_from: String,
    pub tvalue: POSSIBLETOKENVALUE,
    pub ttype: TOKENS,
}
