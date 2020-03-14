use std::rc::Rc;
use crate::option;

/// Description of an option.
pub struct Descriptor {
    name: Rc<String>,
    value_type: option::Type,
    description: String,
}

impl Descriptor {
    /// Create a new option descriptor.
    pub fn new(name: &str, value_type: option::Type, description: &str) -> Self {
        Descriptor {
            name: Rc::new(String::from(name)),
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
}
