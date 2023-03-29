package lexer

import (
	"fmt"
	"time"
	"unicode"

	"github.com/xnacly/tiny-interpreter/token"
	"github.com/xnacly/tiny-interpreter/util"
)

var keywords = map[string]token.TokenType{
	"and":    token.AND,
	"or":     token.OR,
	"fun":    token.FUN,
	"for":    token.FOR,
	"if":     token.IF,
	"else":   token.ELSE,
	"true":   token.TRUE,
	"false":  token.FALSE,
	"nil":    token.NIL,
	"return": token.RETURN,
	"var":    token.VAR,
	"while":  token.WHILE,
	"class":  token.CLASS,
	"super":  token.SUPER,
	"this":   token.THIS,
	"print":  token.PRINT,
}

type Lexer struct {
	input       []rune // the input to lex
	position    int    // current position in input (points to current char)
	currentChar rune   // current character
	line        int    // position in the file
	line_pos    int    // position in the current line
}

// creates a new instance of Lexer and returns it
func NewLexer(input string) *Lexer {
	l := &Lexer{input: []rune(input), line: 1, line_pos: 0, position: 0}
	l.currentChar = l.input[l.position]
	return l
}

// advances the lexer by one character
func (l Lexer) advance() {
	if l.position+1 < len(l.input) {
		l.position++
		l.line_pos++
		l.currentChar = l.input[l.position]
	} else {
		l.currentChar = 0
	}
}

// returns true if the current character is EOF or the length of the input is reached
func (l Lexer) atEnd() bool {
	return l.position >= len(l.input) || l.currentChar == 0
}

// returns the next character without advancing the lexer
func (l Lexer) peek() rune {
	if l.atEnd() {
		return 0
	} else {
		return l.input[l.position+1]
	}
}

// returns true if the next character is equal to c
func (l Lexer) peek_equals(c rune) bool {
	return c == l.peek()
}

// returns true if the current character is in the alphabet
func (l Lexer) isAlpha() bool {
	return unicode.IsLetter(l.currentChar)
}

// returns true if the current character is in the alphabet or is a digit
func (l Lexer) isAlphaNumeric() bool {
	return l.isAlpha() || unicode.IsDigit(l.currentChar)
}

// advances the lexer until the current character is a closing quote, returns the string, throws an error if the end of the input is reached before closing the string
func (l Lexer) string() token.Token {
	return token.Token{}
}

// advances the lexer until the current character is not a digit, returns the digit
func (l Lexer) number() token.Token {
	return token.Token{}
}

// advances the lexer until the current character is a whitespace, returns the identifier
func (l Lexer) identifier() token.Token {
	return token.Token{}
}

// lexes the input and returns a slice of tokens
func (l Lexer) Lex() []token.Token {
	startTime := time.Now()
	tokens := []token.Token{}
	util.LInfo(fmt.Sprintf("lexed: %d tokens, took %s", len(tokens), time.Since(startTime).String()))
	return tokens
}
