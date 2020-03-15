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


## Example

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
