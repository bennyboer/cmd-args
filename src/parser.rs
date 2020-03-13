use std::collections::HashMap;
use std::result;
use std::rc::Rc;
use crate::{OptionDescriptor, Group, OptionType, OptionValue, ArgDescriptor, ArgValue};
use crate::error::ParserError;

/// Parser for command line options and arguments.
pub struct Parser {
    /// Map of anticipated options during parsing.
    anticipated_options: HashMap<Rc<String>, Rc<OptionDescriptor>>,
}

/// Type alias for parser results.
pub type Result<T> = result::Result<T, ParserError>;

impl Parser {
    const OPTION_PREFIX: char = '-';
    const OPTION_KEY_VALUE_SPLIT: char = '=';
    const HELP_OPTION: &'static str = "help";

    pub fn new() -> Parser {
        Parser {
            anticipated_options: HashMap::new(),
        }
    }

    pub fn parse(&mut self, mut group: Group, args: &[String]) -> Result<()> {
        self.anticipated_options.clear();

        // TODO Refactor big method

        // Add help option to anticipated options.
        let help_option_descriptor = OptionDescriptor::new(Parser::HELP_OPTION, OptionType::Bool { default: false }, "Get this information displayed");
        self.anticipated_options.insert(help_option_descriptor.take_name(), Rc::new(help_option_descriptor));

        // Save root groups options.
        for entry in group.get_options() {
            self.anticipated_options.insert(Rc::clone(entry.0), Rc::clone(entry.1));
        }

        // Find command context (via specified groups).
        let mut cur_group = &mut group;
        let mut args_pos = 1;
        for arg in &args[1..] {
            if cur_group.has_child_name(arg) {
                cur_group = cur_group.get_mut_child(arg);

                // Save current groups options.
                for entry in cur_group.get_options() {
                    if self.anticipated_options.contains_key(entry.0.as_ref()) {
                        return Err(ParserError {
                            message: format!("Option '{}' declared multiple times in group specifications", entry.0)
                        });
                    }
                    self.anticipated_options.insert(Rc::clone(entry.0), Rc::clone(entry.1));
                }
            } else {
                break; // Command context path found -> Continue with option and argument parsing.
            }

            args_pos += 1;
        }

        // Parse options and get arguments
        let mut option_value_lookup = HashMap::new();
        let mut arguments = Vec::new();
        let mut skip_next = false;
        for i in args_pos..args.len() {
            if skip_next {
                skip_next = false;
                continue;
            }

            let arg = &args[i];

            let is_option = arg.starts_with(Parser::OPTION_PREFIX);

            if is_option {
                let is_key_value_option = arg.contains(Parser::OPTION_KEY_VALUE_SPLIT);
                let option = self.parse_option(
                    arg,
                    if args.len() > i + 1 { Some(&args[i + 1]) } else { None },
                    is_key_value_option,
                )?;

                if !is_key_value_option {
                    skip_next = true;
                }
                option_value_lookup.insert(String::from(option.0), option.1);
            } else {
                arguments.push(arg);
            }
        }

        // Fill option value lookup with default values for options that have not been specified
        for entry in &self.anticipated_options {
            if !option_value_lookup.contains_key(entry.0.as_ref()) {
                option_value_lookup.insert(entry.0.to_string(), OptionValue::from_default(entry.1.value_type()));
            }
        }

        // Save argument descriptors for later.
        let arg_descriptors = cur_group.take_arguments();

        // Show help if specified as option
        if let OptionValue::Bool { value } = option_value_lookup.get(Parser::HELP_OPTION).unwrap() {
            if *value {
                self.show_help(cur_group, &self.anticipated_options, &arg_descriptors);
                return Ok(());
            }
        }

        // Parse arguments
        if arguments.len() != arg_descriptors.len() {
            return Err(ParserError {
                message: format!("Expected to have {} arguments but got {}", arg_descriptors.len(), arguments.len())
            });
        }

        let mut argument_values = Vec::new();
        for i in 0..arguments.len() {
            let descriptor = &arg_descriptors[i];
            let arg = arguments[i];

            // Check if argument is parsable using the argument descriptor information
            let value = match ArgValue::parse(descriptor.value_type(), arg) {
                Ok(v) => v,
                Err(_) => return Err(ParserError {
                    message: format!("Expected argument '{}' at position {} to be of type '{}'", arg, i + 1, descriptor.value_type())
                })
            };

            argument_values.push(value);
        }

        // Call group consumer.
        cur_group.call_consumer(argument_values, option_value_lookup);
        Ok(())
    }

    /// Show help for the passed group configuration.
    fn show_help(&self, group: &Group, option_descriptors: &HashMap<Rc<String>, Rc<OptionDescriptor>>, arg_descriptors: &Vec<ArgDescriptor>) {
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

    /// Get the option type for the passed option name.
    fn get_option_type_for_name(&self, option_name: &str) -> Result<&OptionType> {
        match self.anticipated_options.get(&String::from(option_name)) {
            Some(o) => Ok(o.value_type()),
            None => Err(ParserError {
                message: format!("Option '--{}' is unknown in the command context", option_name)
            })
        }
    }

    fn parse_option<'a>(&self, option_arg: &'a str, next_arg: Option<&str>, is_key_value_option: bool) -> Result<(&'a str, OptionValue)> {
        let raw_option = option_arg.trim_start_matches(Parser::OPTION_PREFIX); // Strip leading '-' chars

        // Check if option is in key-value form '--OPTION_NAME=OPTION_VALUE'
        let (option_name, option_value) = if is_key_value_option {
            let parts: Vec<&str> = raw_option.split(Parser::OPTION_KEY_VALUE_SPLIT).collect();
            (parts[0], parts[1])
        } else {
            let potential_value = next_arg;

            let is_option_without_value = potential_value.is_none() || potential_value.unwrap().starts_with(Parser::OPTION_PREFIX);
            let option_value = if is_option_without_value {
                // Option without value! Only allowed for boolean options.
                let option_type = self.get_option_type_for_name(raw_option)?;

                if let OptionType::Bool { default: _ } = option_type {
                    "true"
                } else {
                    return Err(ParserError {
                        message: format!("Encountered option '{}' without value that is not of type boolean. Specify a value for the option.", raw_option)
                    });
                }
            } else {
                potential_value.unwrap()
            };

            (raw_option, option_value)
        };

        let option_type = match self.get_option_type_for_name(option_name) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let option_value = match OptionValue::parse(option_type, option_value) {
            Ok(v) => v,
            Err(_) => return Err(ParserError {
                message: format!("Expected value '{}' of option '--{}' to be of type '{}'", option_value, option_name, option_type)
            })
        };

        Ok((option_name, option_value))
    }
}
