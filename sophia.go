package main

import (
	"bufio"
	"os"

	"github.com/xnacly/tiny-interpreter/lexer"
	"github.com/xnacly/tiny-interpreter/logger"
	"github.com/xnacly/tiny-interpreter/util"
)

// get cli arguments
// check if input file, if true interpret, if false start repl
// check if output file, if false print to stdout, if true write to file

func main() {
	args := util.Arguments()
	if args.InFile == "" {
		logger.LInfo("No input file specified. switching to repl mode")
		for {
			scanner := bufio.NewScanner(os.Stdin)
			logger.LPrompt()
			for scanner.Scan() {
				line := scanner.Text()
				if len(line) == 0 {
					logger.LPrompt()
					continue
				} else if line[0] == '.' {
					switch line[1:] {
					case "exit":
						logger.LInfo("Exiting...")
						os.Exit(0)
					default:
						logger.LWarn("Unknown command: '" + line[1:] + "'")
					}
				} else {
					l := lexer.NewLexer(line)
					tokens := l.Lex()
					logger.LTokens(tokens)
				}
				logger.LPrompt()
			}
		}
	} else {
		content, err := os.ReadFile(args.InFile)
		if err != nil {
			logger.LError("couldn't open '" + args.InFile + "': " + err.Error())
		}
		l := lexer.NewLexer(string(content))
		tokens := l.Lex()
		logger.LTokens(tokens)
	}
}
