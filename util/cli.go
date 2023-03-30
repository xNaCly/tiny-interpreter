// contains everything regarding the command line interface of teo
package util

import (
	"os"
)

type SophiaArguments struct {
	InFile string
}

// defines and parses cli arguments
func Arguments() SophiaArguments {
	args := os.Args
	if len(args) >= 2 {
		return SophiaArguments{
			InFile: args[1],
		}
	}
	return SophiaArguments{}
}
