use crate::arg::Type;

/// Descriptor for anticipated arguments.
pub struct Descriptor {
    /// Anticipated value type for the argument.
    value_type: Type,

    /// Description of the argument.
    description: String,
}

impl Descriptor {
    /// Create a new argument descriptor.
    pub fn new(value_type: Type, description: &str) -> Self {
        Descriptor {
            value_type,
            description: String::from(description),
        }
    }

    /// Get the type of the argument value.
    pub fn value_type(&self) -> &Type {
        &self.value_type
    }

    /// Get the description of the argument.
    pub fn description(&self) -> &String {
        &self.description
    }
}
