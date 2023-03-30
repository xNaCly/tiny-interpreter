package token

type TokenType int

type Token struct {
	Type    TokenType
	Pos     int
	Literal string
	Value   any
	Line    int
}

const (
	EOF = iota + 1
	UNKNOWN

	// single char token
	PLUS
	MINUS
	COMMA
	SEMICOLON
	SLASH
	ASTERISK
	DOT

	// one or two char token
	MOD
	OPENPAREN
	CLOSEPAREN
	RIGHTBRACE
	LEFTBRACE
	BANG
	BANGEQUAL
	EQUAL
	LESSTHAN
	GREATERTHAN
	LESSTHANEQUAL
	GREATERTHANEQUAL
	EXPONENT

	// literals
	IDENTIFIER
	INTEGER
	FLOAT
	STRING

	// keywords
	AND
	OR
	FUN
	FOR
	IF
	ELSE
	TRUE
	FALSE
	NIL
	RETURN
	VAR
	WHILE
	CLASS
	SUPER
	THIS
	PRINT
)

var KEYWORDS = map[string]TokenType{
	"and":    AND,
	"or":     OR,
	"fun":    FUN,
	"for":    FOR,
	"if":     IF,
	"else":   ELSE,
	"true":   TRUE,
	"false":  FALSE,
	"nil":    NIL,
	"return": RETURN,
	"var":    VAR,
	"while":  WHILE,
	"class":  CLASS,
	"super":  SUPER,
	"this":   THIS,
	"print":  PRINT,
}

var TOKEN_LOOKUP = map[TokenType]string{
	EOF:              "EOF",
	UNKNOWN:          "UNKNOWN",
	PLUS:             "PLUS",
	MINUS:            "MINUS",
	COMMA:            "COMMA",
	SEMICOLON:        "SEMICOLON",
	SLASH:            "SLASH",
	ASTERISK:         "ASTERISK",
	DOT:              "DOT",
	MOD:              "MOD",
	OPENPAREN:        "OPENPAREN",
	CLOSEPAREN:       "CLOSEPAREN",
	RIGHTBRACE:       "RIGHTBRACE",
	LEFTBRACE:        "LEFTBRACE",
	BANG:             "BANG",
	BANGEQUAL:        "BANGEQUAL",
	EQUAL:            "EQUAL",
	LESSTHAN:         "LESSTHAN",
	GREATERTHAN:      "GREATERTHAN",
	LESSTHANEQUAL:    "LESSTHANEQUAL",
	GREATERTHANEQUAL: "GREATERTHANEQUAL",
	EXPONENT:         "EXPONENT",
	IDENTIFIER:       "IDENTIFIER",
	INTEGER:          "INTEGER",
	FLOAT:            "FLOAT",
	STRING:           "STRING",
	AND:              "AND",
	OR:               "OR",
	FUN:              "FUN",
	FOR:              "FOR",
	IF:               "IF",
	ELSE:             "ELSE",
	TRUE:             "TRUE",
	FALSE:            "FALSE",
	NIL:              "NIL",
	RETURN:           "RETURN",
	VAR:              "VAR",
	WHILE:            "WHILE",
	CLASS:            "CLASS",
	SUPER:            "SUPER",
	THIS:             "THIS",
	PRINT:            "PRINT",
}
