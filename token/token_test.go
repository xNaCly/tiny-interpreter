package token

import (
	"testing"
)

func TestToken(t *testing.T) {
	token := Token{
		Type:    EOF,
		Literal: "",
		Pos:     0,
		Line:    0,
	}
	if token.Type != EOF {
		t.Errorf("token type is not 'EOF'")
	}
}

func TestTokenType(t *testing.T) {
	token := Token{
		Type:    PLUS,
		Literal: "+",
		Pos:     0,
		Line:    0,
	}
	if token.Type != PLUS {
		t.Errorf("token type is not PLUS")
	}
}
