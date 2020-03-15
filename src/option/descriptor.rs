use std::rc::Rc;
use crate::option;
use std::collections::HashSet;

/// Description of an option.
pub struct Descriptor {
    name: Rc<String>,
    aliases: HashSet<String>,
    value_type: option::Type,
    description: String,
}

impl Descriptor {
    /// Create a new option descriptor.
    pub fn new(name: &str, value_type: option::Type, description: &str) -> Self {
        Descriptor {
            name: Rc::new(String::from(name)),
            aliases: HashSet::new(),
            value_type,
            description: String::from(description),
        }
    }

    /// Get the name of the option.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Take a reference to the name of the option.
    pub fn take_name(&self) -> Rc<String> {
        Rc::clone(&self.name)
    }

    /// Get the type of the option value.
    pub fn value_type(&self) -> &option::Type {
        &self.value_type
    }

    /// Get the description of the option.
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Add an alias to the option.
    pub fn add_alias(mut self, alias: &str) -> Self {
        self.aliases.insert(String::from(alias));

        self
    }

    /// Get aliases.
    pub fn get_aliases(&self) -> &HashSet<String> {
        &self.aliases
    }
}
