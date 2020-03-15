use crate::{Group, option, arg};
use std::rc::Rc;

/// Entry in the help documentation.
pub struct HelpEntry<K, V> {
    pub key: K,
    pub value: V,
}

/// Help formatter to use when printing the help documentation.
pub trait HelpPrinter {
    /// Print the help documentation.
    fn print(
        &self,
        group: &Group,
        subcommand_entries: &Vec<HelpEntry<&Rc<String>, &Rc<Group>>>,
        option_entries: &Vec<HelpEntry<&Rc<String>, &Rc<option::Descriptor>>>,
        arg_entries: &Vec<arg::Descriptor>,
    );
}

/// Default help printer used when none is specified.
pub struct DefaultHelpPrinter {}

impl HelpPrinter for DefaultHelpPrinter {
    fn print(
        &self,
        group: &Group,
        subcommand_entries: &Vec<HelpEntry<&Rc<String>, &Rc<Group>>>,
        option_entries: &Vec<HelpEntry<&Rc<String>, &Rc<option::Descriptor>>>,
        arg_entries: &Vec<arg::Descriptor>,
    ) {
        println!("\n### DESCRIPTION ###");
        println!("{description}", description = group.description());

        println!("\n### SUB-COMMANDS ###");
        if subcommand_entries.len() == 0 {
            println!("(No sub-commands available...)");
        } else {
            // Get longest sub-command name
            let mut max_length = 0;
            for entry in subcommand_entries {
                let prefix = match group.get_aliases_for_group_name(&entry.key) {
                    Some(aliases) => format!(
                        "{name} ({aliases})",
                        name = entry.key,
                        aliases = aliases.iter().map(|s| s.as_ref().to_string()).collect::<Vec<String>>().join(", ")
                    ),
                    None => entry.key.to_string(),
                };
                if prefix.len() > max_length {
                    max_length = prefix.len();
                }
            }

            for entry in subcommand_entries {
                let prefix = match group.get_aliases_for_group_name(&entry.key) {
                    Some(aliases) => format!(
                        "{name} ({aliases})",
                        name = entry.key,
                        aliases = aliases.iter().map(|s| s.as_ref().to_string()).collect::<Vec<String>>().join(", ")
                    ),
                    None => entry.key.to_string(),
                };
                println!("  - {prefix:<width$} | {description}", prefix = prefix, width = max_length, description = entry.value.description());
            }
        }

        println!("\n### OPTIONS ###");
        if option_entries.len() == 0 {
            println!("(No options available...)");
        } else {
            // Get longest option name
            let mut max_length = 0;
            for entry in option_entries {
                let aliases = entry.value.get_aliases();
                let prefix = if aliases.len() == 0 {
                    format!("{name} <{type_name}>", name = entry.key, type_name = entry.value.value_type())
                } else {
                    format!(
                        "{name} ({aliases}) <{type_name}>",
                        name = entry.key,
                        aliases = aliases.iter().map(|s| format!("-{}", s)).collect::<Vec<String>>().join(", "),
                        type_name = entry.value.value_type()
                    )
                };
                if prefix.len() > max_length {
                    max_length = prefix.len();
                }
            }

            for entry in option_entries {
                let aliases = entry.value.get_aliases();
                let prefix = if aliases.len() == 0 {
                    format!("{name} <{type_name}>", name = entry.key, type_name = entry.value.value_type())
                } else {
                    format!(
                        "{name} ({aliases}) <{type_name}>",
                        name = entry.key,
                        aliases = aliases.iter().map(|s| format!("-{}", s)).collect::<Vec<String>>().join(", "),
                        type_name = entry.value.value_type()
                    )
                };
                println!("  --{prefix:<width$} | {description}", prefix = prefix, width = max_length, description = entry.value.description());
            }
        }

        println!("\n### ARGUMENTS ###");
        if arg_entries.len() == 0 {
            println!("(Command expects no arguments...)");
        } else {
            let mut max_length = 0;
            for i in 0..arg_entries.len() {
                let arg_d = &arg_entries[i];

                let prefix = format!("{num}. <{type_name}>", num = i + 1, type_name = arg_d.value_type());
                if prefix.len() > max_length {
                    max_length = prefix.len();
                }
            }

            for i in 0..arg_entries.len() {
                let arg_d = &arg_entries[i];

                let prefix = format!("{num}. <{type_name}>", num = i + 1, type_name = arg_d.value_type());
                println!("  {prefix:<width$} | {description}", prefix = prefix, width = max_length, description = arg_d.description());
            }
        }

        println!();
    }
}
