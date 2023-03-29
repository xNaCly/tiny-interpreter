// contains everything regarding the command line interface of teo
package util

import (
	"os"
)

type TeoArguments struct {
	InFile string
}

// defines and parses cli arguments
func Arguments() TeoArguments {
	args := os.Args
	if len(args) < 2 {
		LInfo("No input file specified. switching to repl mode")
	} else {
		return TeoArguments{
			InFile: args[1],
		}
	}
	return TeoArguments{}
}
