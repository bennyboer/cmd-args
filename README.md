# `cmd-args` Command line parser for Rust

`cmd-args` is a command line parser for Rust, which can be used to build command line interfaces easily.


## Introduction

You might have noticed that in the rust-ecosystem there are plenty of command line parser solutions available by now, so why bother and write another?
I started this project because I was writing a small Rust program in order to learn Rust.
For this project I ended up needing a command line parser and thought that it would not hurt to just try writing one by myself.
It ended up working just fine, thus it is now available here for others who might enjoy using it.


## Concepts

### Command context (Group)

The command line parser is parsing a command context (for example for the call `scoop bucket add` the command context would be the sub-command `add` of the `bucket` command of the program `scoop`).
That context is used to determine the anticipated/allowed options and arguments the command is taking.
The struct in the library used to define a command context is called `Group`.


### Options (Flags are also options!)

For `cmd-args` options are **all** optional arguments which have one or multiple `-` characters as prefix.
That means **flags** are options with `boolean` type.

The parser will not care how many `-` chars you specify, so `-?`, `--?` and `--------?` will be the interpreted the same.

When having multiple sub-commands all involved options on the command context path from the root `Group` to the leaf `Group` are accepted.
For example when having an app accepting `scoop bucket add`, which means three levels (Root, intermediate and leaf), will accept all options specified on the root `Group` `scoop`, the intermediate level `Group` `bucket` and the leaf level `Group` `add`.


### Arguments

Arguments are defined by the current command context.
When calling for example a app `scoop bucket add` and the command context (`Group`) for `add` takes one argument, then the whole call only accepts one argument (Compare with options where for each command context (`Group`) level the options are accepted).


## Features

### Nested commands (Groups)

You might want to build a CLI with nested commands meaning commands which have sub-commands, which may have sub-commands, which may have sub-commands, ... (You see where this going!).

Take for example a CLI application named `store` which does have sub-commands `shelve, staff, ...`. 
When using sub-command `shelve` we have another three sub-commands `list, add, remove` to list all available shelves, add another shelve, remove a shelve.

These are nested commands! You can go creative and define arbitrarily nested CLIs!


### Generated help

Command line interfaces most likely have a documentation available when calling them using the `--help` or `-?` flag.
That one is automatically generated from the command specification!


### Sub-command and option aliases

The library supports aliasing sub-commands and options which will as well appear in the auto-generated help documentation.


## Example

> More examples coming soon in the repositories `example` directory!

```rust
let group = Group::new(Box::new(|args, options| {
    let test_argument = args[0].str().unwrap();
    let the_truth = options.get("the-truth").unwrap().int().unwrap();

    println!("Hello from root command with test argument value '{}' and the_truth = '{}'", test_argument, the_truth);
}), "Simple CLI tool")
    .add_option(option::Descriptor::new("the-truth", option::Type::Int { default: 42 }, "The truth about everything"))
    .add_argument(arg::Descriptor::new(arg::Type::Str, "Test text"))
    .add_child("subcommand", None, Group::new(Box::new(|args, options| {
        println!("Hello from subcommand!");
    }), "A sub command!"));

parser::parse(group, None).unwrap();
```
