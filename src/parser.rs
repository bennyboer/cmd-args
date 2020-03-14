use std::collections::HashMap;
use std::result;
use std::rc::Rc;
use crate::error::ParserError;
use crate::Group;
use crate::option;
use crate::arg;

/// Type alias for parser results.
pub type Result<T> = result::Result<T, ParserError>;

static OPTION_PREFIX: char = '-';
static OPTION_KEY_VALUE_SPLIT: char = '=';
static HELP_OPTION: &'static str = "help";

/// Parse the passed command line arguments using the passed group.
pub fn parse(mut group: Group, args: &[&str]) -> Result<()> {
    let (ctx_group, anticipated_options, parse_start_pos) = prepare_parsing_context(&mut group, args)?;
    let arg_descriptors = ctx_group.take_arguments();

    let (raw_options, raw_arguments) = split_raw_arguments(&args[parse_start_pos..], &anticipated_options)?;

    let mut option_value_lookup = parse_options(raw_options, &anticipated_options)?;
    fill_default_options(&mut option_value_lookup, &anticipated_options);

    // Show help if specified as option
    if let option::Value::Bool { value } = option_value_lookup.get(HELP_OPTION).unwrap() {
        if *value {
            show_help(&ctx_group, &anticipated_options, &arg_descriptors);
            return Ok(());
        }
    }

    let argument_values = parse_arguments(&arg_descriptors, raw_arguments)?;

    // Call group consumer.
    ctx_group.take_consumer()(argument_values, option_value_lookup);
    Ok(())
}

/// Prepare the parsing context for the passed group and arguments.
/// Returns the group context, anticipated options to parse as well as the rest of the raw
/// command line arguments to parse.
fn prepare_parsing_context<'a>(group: &'a mut Group, args: &[&str]) -> Result<(&'a mut Group, HashMap<String, Rc<option::Descriptor>>, usize)> {
    let mut anticipated_options: HashMap<String, Rc<option::Descriptor>> = HashMap::new();

    // Add help option to anticipated options.
    let help_option_descriptor = option::Descriptor::new(HELP_OPTION, option::Type::Bool { default: false }, "Get this information displayed");
    anticipated_options.insert(help_option_descriptor.name().clone(), Rc::new(help_option_descriptor));

    // Save root groups options.
    for (option_name, option_descriptor) in group.get_options() {
        anticipated_options.insert(option_name.as_ref().clone(), Rc::clone(option_descriptor));
    }

    // Find command context (via specified groups).
    let mut cur_group = group;
    let mut args_pos = 1;

    for arg in &args[1..] {
        let arg = *arg;

        if cur_group.has_child_name(arg) {
            cur_group = cur_group.get_mut_child(arg);

            // Save current groups options.
            for (option_name, option_descriptor) in cur_group.get_options() {
                if anticipated_options.contains_key(option_name as &str) {
                    return Err(ParserError {
                        message: format!("Option '{}' declared multiple times in group specifications", option_name)
                    });
                }
                anticipated_options.insert(option_name.as_ref().clone(), Rc::clone(option_descriptor));
            }
        } else {
            break; // Command context path found -> Continue with option and argument parsing.
        }

        args_pos += 1;
    }

    Ok((cur_group, anticipated_options, args_pos))
}

/// Get the option type for the passed option name.
fn get_option_type_for_name<'a>(option_name: &str, anticipated_options: &'a HashMap<String, Rc<option::Descriptor>>) -> Result<&'a option::Type> {
    match anticipated_options.get(option_name) {
        Some(o) => Ok(o.value_type()),
        None => Err(ParserError {
            message: format!("Option '--{}' is unknown in the command context", option_name)
        })
    }
}

/// Check whether the passed raw argument string is a option.
fn is_option(raw_arg: &str) -> bool {
    raw_arg.starts_with(OPTION_PREFIX)
}

/// Split the passed raw command line arguments into options (name and value) and arguments.
fn split_raw_arguments<'a>(args: &[&'a str], anticipated_options: &HashMap<String, Rc<option::Descriptor>>) -> Result<(HashMap<&'a str, &'a str>, Vec<&'a str>)> {
    let mut raw_options = HashMap::new();
    let mut raw_arguments = Vec::new();

    let mut skip_next = false;
    for i in 0..args.len() {
        if skip_next {
            skip_next = false;
            continue;
        }

        let arg = args[i];

        if is_option(arg) {
            let raw_option = arg.trim_start_matches(OPTION_PREFIX); // Strip leading '-' chars

            let is_key_value_option = arg.contains(OPTION_KEY_VALUE_SPLIT);
            let (option_name, option_value) = if is_key_value_option {
                // Value is in same string separated by '='
                let parts: Vec<&str> = raw_option.split(OPTION_KEY_VALUE_SPLIT).collect();
                (parts[0], parts[1])
            } else {
                // Value is in next raw command line argument
                let next_arg = if args.len() > i + 1 { Some(&args[i + 1]) } else { None };

                let is_option_without_value = next_arg.is_none()
                    || is_option(next_arg.unwrap());

                let option_value = if is_option_without_value {
                    // Option without value! Only allowed for boolean options.
                    let option_type = get_option_type_for_name(raw_option, anticipated_options)?;

                    match option_type {
                        option::Type::Bool { default: _ } => "true",
                        _ => return Err(ParserError {
                            message: format!("Encountered option '{}' without value that is not of type boolean. Specify a value for the option.", raw_option)
                        })
                    }
                } else {
                    next_arg.unwrap()
                };

                skip_next = true; // Skip the next raw command line argument since it was already processed

                (raw_option, option_value)
            };

            raw_options.insert(option_name, option_value);
        } else {
            raw_arguments.push(arg);
        }
    }

    Ok((raw_options, raw_arguments))
}

/// Parse raw options to their actual values.
fn parse_options<'a>(raw_options: HashMap<&'a str, &'a str>, anticipated_options: &HashMap<String, Rc<option::Descriptor>>) -> Result<HashMap<&'a str, option::Value>> {
    let mut option_value_lookup = HashMap::new();

    for (option_name, raw_value) in raw_options.into_iter() {
        option_value_lookup.insert(option_name, parse_option(option_name, raw_value, anticipated_options)?);
    }

    Ok(option_value_lookup)
}

/// Parse the passed option (name and raw value).
fn parse_option(name: &str, raw_value: &str, anticipated_options: &HashMap<String, Rc<option::Descriptor>>) -> Result<option::Value> {
    let option_type = match get_option_type_for_name(name, anticipated_options) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    Ok(match option::Value::parse(option_type, raw_value) {
        Ok(v) => v,
        Err(_) => return Err(ParserError {
            message: format!("Expected value '{}' of option '--{}' to be of type '{}'", raw_value, name, option_type)
        })
    })
}

/// Add all missing options in the lookup with default values.
fn fill_default_options<'a>(option_value_lookup: &mut HashMap<&'a str, option::Value>, anticipated_options: &'a HashMap<String, Rc<option::Descriptor>>) {
    for (option_name, descriptor) in anticipated_options {
        if !option_value_lookup.contains_key(option_name as &str) {
            option_value_lookup.insert(option_name, option::Value::from_default(descriptor.value_type()));
        }
    }
}

/// Parse the passed raw command line arguments to their actual argument values.
fn parse_arguments(descriptors: &Vec<arg::Descriptor>, raw_arguments: Vec<&str>) -> Result<Vec<arg::Value>> {
    if raw_arguments.len() != descriptors.len() {
        return Err(ParserError {
            message: format!("Expected to have {} arguments but got {}", descriptors.len(), raw_arguments.len())
        });
    }

    let mut argument_values = Vec::with_capacity(raw_arguments.len());
    for i in 0..raw_arguments.len() {
        let desc = &descriptors[i];
        let arg = raw_arguments[i];

        // Check if argument is parsable using the argument descriptor information
        let value = match arg::Value::parse(desc.value_type(), arg) {
            Ok(v) => v,
            Err(_) => return Err(ParserError {
                message: format!("Expected argument '{}' at position {} to be of type '{}'", arg, i + 1, desc.value_type())
            })
        };

        argument_values.push(value);
    }

    Ok(argument_values)
}

/// Show help for the passed group configuration.
fn show_help(group: &Group, option_descriptors: &HashMap<String, Rc<option::Descriptor>>, arg_descriptors: &Vec<arg::Descriptor>) {
    println!("\n### DESCRIPTION ###");
    println!("{description}", description = group.description());

    println!("\n### COMMANDS ###");

    let child_groups = group.get_children();

    if child_groups.len() == 0 {
        println!("(No commands available...)");
    } else {
        // Get longest group name
        let mut max_length = 0;
        for g in child_groups {
            if g.0.len() > max_length {
                max_length = g.0.len();
            }
        }

        for g in child_groups {
            println!("  - {name:<width$} | {description}", name = g.0, width = max_length, description = g.1.description());
        }
    }

    println!("\n### OPTIONS ###");

    if option_descriptors.len() == 0 {
        println!("(No options available...)");
    } else {
        // Get longest option name
        let mut max_length = 0;
        for o in option_descriptors {
            let prefix = format!("{name} <{type_name}>", name = o.0, type_name = o.1.value_type());
            if prefix.len() > max_length {
                max_length = prefix.len();
            }
        }

        for o in option_descriptors {
            let prefix = format!("{name} <{type_name}>", name = o.0, type_name = o.1.value_type());
            println!("  --{prefix:<width$} | {description}", prefix = prefix, width = max_length, description = o.1.description());
        }
    }

    println!("\n### ARGUMENTS ###");

    if arg_descriptors.len() == 0 {
        println!("(No arguments available...");
    } else {
        let mut max_length = 0;
        for i in 0..arg_descriptors.len() {
            let arg_d = &arg_descriptors[i];

            let prefix = format!("{num}. <{type_name}>", num = i + 1, type_name = arg_d.value_type());
            if prefix.len() > max_length {
                max_length = prefix.len();
            }
        }

        for i in 0..arg_descriptors.len() {
            let arg_d = &arg_descriptors[i];

            let prefix = format!("{num}. <{type_name}>", num = i + 1, type_name = arg_d.value_type());
            println!("  {prefix:<width$} | {description}", prefix = prefix, width = max_length, description = arg_d.description());
        }
    }

    println!();
}
