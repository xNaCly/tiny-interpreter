package lexer

import (
	"errors"
	"fmt"
	"time"
	"unicode"

	"github.com/xnacly/tiny-interpreter/token"
	"github.com/xnacly/tiny-interpreter/util"
)

var KEYWORDS = map[string]token.TokenType{
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
func (l *Lexer) advance() {
	if l.position+1 < len(l.input) {
		l.position++
		l.line_pos++
		l.currentChar = l.input[l.position]
	} else {
		l.currentChar = 0
	}
}

// returns true if the current character is EOF or the length of the input is reached
func (l *Lexer) atEnd() bool {
	return l.position >= len(l.input) || l.currentChar == 0
}

// returns the next character without advancing the lexer
func (l *Lexer) peek() rune {
	if l.atEnd() {
		return 0
	} else {
		return l.input[l.position+1]
	}
}

// returns true if the next character is equal to c
func (l *Lexer) peekEquals(c rune) bool {
	return c == l.peek()
}

// returns true if the current character is in the alphabet
func (l *Lexer) isAlpha() bool {
	return unicode.IsLetter(l.currentChar)
}

// returns true if the current character is in the alphabet or is a digit
func (l *Lexer) isAlphaNumeric() bool {
	return l.isAlpha() || unicode.IsDigit(l.currentChar)
}

// advances the lexer until the current character is a closing quote, returns the string, throws an error if the end of the input is reached before closing the string
func (l *Lexer) string() (string, error) {
	res := ""
	for l.currentChar != '"' && !l.atEnd() {
		res += string(l.currentChar)
		l.advance()
	}

	if l.atEnd() {
		return "", errors.New("unterminated string")
	} else {
		return res, nil
	}
}

// advances the lexer until the current character is not a digit, returns the digit
func (l *Lexer) number() (string, int, error) {
	return "", 0, errors.New("not implemented")
}

// advances the lexer until the current character is a whitespace, returns the identifier
func (l *Lexer) identifier() (token.TokenType, string, error) {
	res := ""
	for l.isAlphaNumeric() {
		res += string(l.currentChar)
		l.advance()
	}

	if val, ok := KEYWORDS[res]; ok {
		return val, res, nil
	} else {
		return token.UNKNOWN, res, errors.New("couldn't parse identifier")
	}
}

// lexes the input and returns a slice of tokens
func (l *Lexer) Lex() []token.Token {
	startTime := time.Now()
	tokens := []token.Token{}
	for !l.atEnd() {
		var kind token.TokenType = token.UNKNOWN
		var literal string = ""
		var value any
		switch l.currentChar {
		case ' ', '\t', '\r':
			l.advance()
			continue
		case '\n':
			l.line++
			l.line_pos = 0
			l.advance()
			continue
		case '"':
			l, err := l.string()
			if err != nil {
				util.LError("Unterminated string")
			}
			literal = l
			value = l
			kind = token.STRING
		case '+':
			kind = token.PLUS
		case '-':
			kind = token.MINUS
		case ',':
			kind = token.COMMA
		case ';':
			kind = token.SEMICOLON
		case '/':
			if !l.peekEquals('/') {
				kind = token.SLASH
			} else {
				for l.currentChar != '\n' && !l.atEnd() {
					l.advance()
					continue
				}
			}
		case '.':
			kind = token.DOT
		case '(':
			kind = token.OPENPAREN
		case ')':
			kind = token.CLOSEPAREN
		case '{':
			kind = token.LEFTBRACE
		case '}':
			kind = token.RIGHTBRACE
		case '%':
			kind = token.MOD
		case '=':
			kind = token.EQUAL
		case '<':
			if l.peekEquals('=') {
				l.advance()
				kind = token.LESSTHANEQUAL
			} else {
				kind = token.EQUAL
			}
		case '>':
			if l.peekEquals('=') {
				l.advance()
				kind = token.GREATERTHANEQUAL
			} else {
				kind = token.GREATERTHAN
			}
		case '!':
			if l.peekEquals('=') {
				l.advance()
				kind = token.BANGEQUAL
			} else {
				kind = token.BANG
			}
		case '*':
			if l.peekEquals('*') {
				l.advance()
				kind = token.BANGEQUAL
			} else {
				kind = token.BANG
			}
		default:
			if unicode.IsDigit(l.currentChar) {
				l, v, err := l.number()
				if err != nil {
					util.LError("couldn't parse integer")
				}
				literal = l
				value = v
				kind = token.INTEGER
			} else if l.isAlpha() {
				t, l, err := l.identifier()
				if err != nil {
					util.LError("couldn't parse identifier")
				}
				literal = l
				kind = t
			} else {
				util.LError("unexpected identifier")
			}
		}
		tokens = append(tokens, token.Token{
			Type:    kind,
			Literal: literal,
			Value:   value,
			Pos:     l.line_pos,
			Line:    l.line,
		})
	}
	tokens = append(tokens, token.Token{
		Type:    token.EOF,
		Pos:     l.line_pos,
		Line:    l.line,
		Literal: "",
	})
	util.LInfo(fmt.Sprintf("(f)lexed: %d tokens, took %s", len(tokens), time.Since(startTime).String()))
	return tokens
}
