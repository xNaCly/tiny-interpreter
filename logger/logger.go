package logger

import (
	"fmt"
	"github.com/xnacly/tiny-interpreter/token"
	"github.com/xnacly/tiny-interpreter/util"
	"log"
)

const (
	ANSI_RESET  = "\033[0m"
	ANSI_RED    = "\033[91m"
	ANSI_YELLOW = "\033[93m"
	ANSI_BLUE   = "\033[36m"
)

// prefixes s with 'info', prints result
func LInfo(s string) {
	log.Printf("%sinfo%s: %s\n", ANSI_BLUE, ANSI_RESET, s)
}

// prefixes s with 'warn', prints result
func LWarn(s string) {
	log.Printf("%swarn%s: %s\n", ANSI_YELLOW, ANSI_RESET, s)
}

// prefixes s with 'error', calls log.Fatalln, prints result, exits with error code 1
func LError(s string) {
	log.Fatalf("%serror%s: %s\n", ANSI_RED, ANSI_RESET, s)
}

// simple call to the log.Println function, only here to keep things isolated and consistent
func L(v ...any) {
	log.Println(v...)
}

func lToken(t token.Token) {
	if t.Type == token.INTEGER || t.Type == token.STRING {
		fmt.Printf("{ type: %s (%d), pos: %d, literal: %s, value: %v, line: %d }\n", token.TOKEN_LOOKUP[t.Type], t.Type, t.Pos, t.Literal, t.Value, t.Line)
	} else if len(t.Literal) != 0 {
		fmt.Printf("{ type: %s (%d), pos: %d, literal: %s, line: %d }\n", token.TOKEN_LOOKUP[t.Type], t.Type, t.Pos, t.Literal, t.Line)
	} else {
		fmt.Printf("{ type: %s (%d), pos: %d, line: %d }\n", token.TOKEN_LOOKUP[t.Type], t.Type, t.Pos, t.Line)
	}
}

// logs the lexed tokens
func LTokens(tokens []token.Token) {
	for _, t := range tokens {
		lToken(t)
	}
}

// logs the util.REPL_PROMPT with the given util.REPL_PROMPT_COLOR
func LPrompt() {
	fmt.Printf("%s%s%s", util.REPL_PROMPT_COLOR, util.REPL_PROMPT, ANSI_RESET)
}
