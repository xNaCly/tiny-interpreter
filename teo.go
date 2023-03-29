package main

import (
	"github.com/xnacly/tiny-interpreter/lexer"
	"github.com/xnacly/tiny-interpreter/util"
)

// get cli arguments
// check if input file, if true interpret, if false start repl
// check if output file, if false print to stdout, if true write to file

func main() {
	args := util.Arguments()
	util.L(args)
	lexer := lexer.NewLexer("var f = \"this is a string\"")
	lexer.Lex()
}
