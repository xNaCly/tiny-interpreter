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
