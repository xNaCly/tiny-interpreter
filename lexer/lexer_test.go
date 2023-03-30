package lexer

import (
	"testing"

	"github.com/xnacly/tiny-interpreter/token"
)

func TestLexerNumber(t *testing.T) {
	lexer := NewLexer("5")
	tokens := lexer.Lex()
	if tokens[0].Type != token.INTEGER && tokens[0].Value != 5 {
		t.Errorf("token type is not VAR or value is not 5")
	}
}

func TestLexerString(t *testing.T) {
	lexer := NewLexer("\"hello\"")
	tokens := lexer.Lex()
	if tokens[0].Type != token.STRING && tokens[0].Value != "hello" {
		t.Errorf("token type is not STRING or value is not \"hello\"")
	}
}

func TestSingleCharTokens(t *testing.T) {
	tokenTypes := []token.TokenType{
		token.PLUS,
		token.MINUS,
		token.COMMA,
		token.SEMICOLON,
		token.SLASH,
		token.ASTERISK,
		token.DOT,
		token.EOF,
	}
	lexer := NewLexer("+-,;/*.")
	tokens := lexer.Lex()
	for i, token := range tokens {
		if token.Type != tokenTypes[i] {
			t.Errorf("expected tokenType '%d', got '%d'", tokenTypes[i], token.Type)
		}
	}
}

func TestMultiCharTokens(t *testing.T) {
	tokenTypes := []token.TokenType{
		token.MOD,
		token.OPENPAREN,
		token.CLOSEPAREN,
		token.LEFTBRACE,
		token.RIGHTBRACE,
		token.BANG,
		token.BANGEQUAL,
		token.EQUAL,
		token.LESSTHAN,
		token.GREATERTHAN,
		token.LESSTHANEQUAL,
		token.GREATERTHANEQUAL,
		token.EXPONENT,
		token.EOF,
	}
	lexer := NewLexer("% () {} ! != = < > <= >= **")
	tokens := lexer.Lex()
	for i, token := range tokens {
		if token.Type != tokenTypes[i] {
			t.Errorf("expected tokenType '%d', got '%d'", tokenTypes[i], token.Type)
		}
	}
}

func TestLiterals(t *testing.T) {
	tokenTypes := []token.TokenType{
		token.IDENTIFIER,
		token.INTEGER,
		token.STRING,
		token.EOF,
	}
	lexer := NewLexer("f 1209.012 \"test\"")
	tokens := lexer.Lex()
	for i, token := range tokens {
		if token.Type != tokenTypes[i] {
			t.Errorf("expected tokenType '%d', got '%d'", tokenTypes[i], token.Type)
		}
	}
}

func TestKeywords(t *testing.T) {
	tokenTypes := []token.TokenType{
		token.AND,
		token.OR,
		token.FUN,
		token.FOR,
		token.IF,
		token.ELSE,
		token.TRUE,
		token.FALSE,
		token.NIL,
		token.RETURN,
		token.VAR,
		token.WHILE,
		token.CLASS,
		token.SUPER,
		token.THIS,
		token.PRINT,
		token.EOF,
	}
	lexer := NewLexer("and or fun for if else true false nil return var while class super this print")
	tokens := lexer.Lex()
	for i, token := range tokens {
		if token.Type != tokenTypes[i] {
			t.Errorf("expected tokenType '%d', got '%d'", tokenTypes[i], token.Type)
		}
	}
}
