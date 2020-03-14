# `cmd-args` Command line parser for Rust

`cmd-args` is a command line parser for Rust, which can be used to build command line interfaces easily.


## Features

### Nested commands (Groups)

You might want to build a CLI with nested commands meaning commands which have sub-commands, which may have sub-commands, which may have sub-commands, ... (You see where this going!).

Take for example a CLI application named `store` which does have sub-commands `shelve, staff, ...`. 
When using sub-command `shelve` we have another three sub-commands `list, add, remove` to list all available shelves, add another shelve, remove a shelve.

These are nested commands! You can go creative and define arbitrarily nested CLIs!


### Generated help

Command line interfaces most likely have a documentation available when calling them using the `--help` flag.
That one is automatically generated from the command specification!


## Examples

> **TODO**: Create simple example here and multiple examples in an example directory.
