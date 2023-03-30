package lexer

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
	"time"
	"unicode"

	"github.com/xnacly/tiny-interpreter/logger"
	"github.com/xnacly/tiny-interpreter/token"
)

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
	if len(input) > 0 {
		l.currentChar = l.input[l.position]
	} else {
		l.currentChar = 0
	}
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
	if l.atEnd() || l.position+1 >= len(l.input) {
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

	l.advance()

	for l.currentChar != '"' && !l.atEnd() {
		res += string(l.currentChar)
		l.advance()
	}

	if l.atEnd() {
		return res, errors.New("unterminated string")
	} else {
		return res, nil
	}
}

// advances the lexer until the current character is not a digit, returns the digit
func (l *Lexer) number() (string, float64, error) {
	res := ""
	for unicode.IsDigit(l.currentChar) {
		res += string(l.currentChar)
		l.advance()
	}
	if l.currentChar == '.' && unicode.IsDigit(l.peek()) {
		res += string(l.currentChar)

		l.advance()

		for unicode.IsDigit(l.currentChar) {
			res += string(l.currentChar)
			l.advance()
		}
	}
	integer, err := strconv.ParseFloat(res, 64)
	return res, integer, err
}

// advances the lexer until the current character is a whitespace, returns the identifier
func (l *Lexer) identifier() (token.TokenType, string) {
	res := ""
	for l.isAlphaNumeric() {
		res += string(l.currentChar)
		l.advance()
	}

	if val, ok := token.KEYWORDS[res]; ok {
		return val, res
	} else {
		return val, res
	}
}

func (l *Lexer) error(message string, description string, lexeme string) {
	errorLine := strings.Split(string(l.input), "\n")[l.line-1]

	linePosFormatted := ""
	if l.line < 9 {
		linePosFormatted = "0" + fmt.Sprint(l.line)
	} else {
		linePosFormatted = fmt.Sprint(l.line)
	}

	fmt.Printf("%serror%s: %s at pos %d of '%s' on line %d\n\n", logger.ANSI_RED, logger.ANSI_RESET, message, l.line_pos, errorLine, l.line)

	upArrows := "^"
	if len(lexeme) > 1 {
		upArrows = strings.Repeat(upArrows, len(lexeme))
	}

	spaces := ""
	if l.line_pos < len(lexeme) {
		spaces = strings.Repeat(" ", l.line_pos)
	} else {
		spaces = strings.Repeat(" ", (l.line_pos-len(lexeme))+6) + upArrows
	}

	fmt.Printf("%s%s |%s %s\n%s%s %s%s\n\n%s\n",
		logger.ANSI_BLUE,
		linePosFormatted,
		logger.ANSI_RESET,
		errorLine,
		logger.ANSI_RED,
		spaces,
		message,
		logger.ANSI_RESET,
		description,
	)
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
			l.line_pos = -1
			l.advance()
			continue
		case 0:
			kind = token.EOF
		case '"':
			lit, err := l.string()
			if err != nil {
				l.error("Unterminated string", "string literals must be terminated with '\"'", lit)
				return []token.Token{}
			}
			literal = lit
			value = lit
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
			if l.peekEquals('/') {
				for l.currentChar != '\n' && !l.atEnd() {
					l.advance()
				}
				continue
			} else {
				kind = token.SLASH
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
				kind = token.LESSTHAN
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
				kind = token.EXPONENT
			} else {
				kind = token.ASTERISK
			}
		default:
			if unicode.IsDigit(l.currentChar) {
				l, v, err := l.number()
				if err != nil {
					logger.LError("couldn't parse integer")
				}
				literal = l
				value = v
				kind = token.INTEGER
			} else if l.isAlpha() {
				t, l := l.identifier()
				literal = l
				kind = t
			} else {
				l.error("Unexpected character '"+string(l.currentChar)+"'", "expected a valid character, such as a number, operator, or parenthesis", string(l.currentChar))
				return []token.Token{}
			}
		}

		pos := l.line_pos
		if pos == 0 {
			pos = 0
		} else if pos < len(literal) {
			pos = 0
		} else {
			pos = l.line_pos - len(literal)
		}

		tokens = append(tokens, token.Token{
			Type:    kind,
			Literal: literal,
			Value:   value,
			Pos:     pos,
			Line:    l.line,
		})
		if kind != token.INTEGER {
			l.advance()
		}
	}
	tokens = append(tokens, token.Token{
		Type:    token.EOF,
		Pos:     l.line_pos + 1,
		Line:    l.line,
		Literal: "",
	})
	logger.LInfo(fmt.Sprintf("(f)lexed: %d tokens, took %s", len(tokens), time.Since(startTime).String()))
	return tokens
}
