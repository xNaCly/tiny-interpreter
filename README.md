# tiny-interpreter

**INFO**: currently only lexing is implemented, you shouldn't use this language 💀

Learning and applying concepts from the building compilers class in go (building an interpreter (soph) for my own programming language, teolang).
The interpreter does not use any external dependencies.

## Setup and running

> requires git and go

```bash
git clone https://github.com/xNaCly/tiny-interpreter.git
# this will start the repl:
go run .
```

### Building

```bash
git clone https://github.com/xNaCly/tiny-interpreter.git
# this will start the repl:
go build -o soph
```

## Interpreter

The interpreter supports a sophisticated scripting language called the `teolang` (file ending `*.teo`)

### Operating modes

#### REPL

To run the interpreter in `read eval print line` mode, simply invoke it by calling the built binary or following the steps above and running it using `go run .`.

#### File mode

To feed the interpreter a file to execute, simply call the interpreter with the file to run as an argument:

```bash
soph file.teo
```

### Command line arguments

#### Help

To view the help invoke the interpreter with either the `-h` or the `--help` flag:

```bash
soph -h
soph --help
go run . -h
go run . -help
```

This readme will probably not be as up to date as the above command. Run the executable and take a look at the help output for more info.

### Examples

Take a look at the `*.teo` files in the [/examples](/examples) directory. To run an example, run the following command:

```
sophia hello_world.teo
```
