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
	return TeoArguments{
		InFile: args[1],
	}
}
