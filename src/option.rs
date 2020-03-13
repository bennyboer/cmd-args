use std::fmt;
use std::rc::Rc;

/// Possible types for a CLI option.
pub enum OptionType {
    Bool { default: bool },
    Str { default: String },
    Int { default: i32 },
    Float { default: f64 },
}

impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            OptionType::Bool { default: _ } => "boolean",
            OptionType::Str { default: _ } => "string",
            OptionType::Int { default: _ } => "integer",
            OptionType::Float { default: _ } => "float",
        })
    }
}

/// Possible values for a CLI option.
pub enum OptionValue {
    Bool { value: bool },
    Str { value: String },
    Int { value: i32 },
    Float { value: f64 },
}

impl OptionValue {
    /// Parse option value from string.
    pub fn parse(option_type: &OptionType, input: &str) -> super::parser::Result<OptionValue> {
        match option_type {
            OptionType::Bool { default: _ } => Ok(OptionValue::Bool {
                value: input.parse()?
            }),
            OptionType::Str { default: _ } => Ok(OptionValue::Str {
                value: String::from(input)
            }),
            OptionType::Int { default: _ } => Ok(OptionValue::Int {
                value: input.parse()?
            }),
            OptionType::Float { default: _ } => Ok(OptionValue::Float {
                value: input.parse()?
            }),
        }
    }

    /// Create option value from default option value.
    pub fn from_default(option_type: &OptionType) -> OptionValue {
        match option_type {
            OptionType::Bool { default } => OptionValue::Bool {
                value: *default
            },
            OptionType::Str { default } => OptionValue::Str {
                value: default.clone()
            },
            OptionType::Int { default } => OptionValue::Int {
                value: *default
            },
            OptionType::Float { default } => OptionValue::Float {
                value: *default
            },
        }
    }
}

impl fmt::Display for OptionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            OptionValue::Bool { value } => value.to_string(),
            OptionValue::Str { value } => value.to_string(),
            OptionValue::Int { value } => value.to_string(),
            OptionValue::Float { value } => value.to_string(),
        })
    }
}

/// Description of an option.
pub struct OptionDescriptor {
    name: Rc<String>,
    value_type: OptionType,
    description: String,
}

impl OptionDescriptor {
    /// Create a new option descriptor.
    pub fn new(name: &str, value_type: OptionType, description: &str) -> Self {
        OptionDescriptor {
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
    pub fn value_type(&self) -> &OptionType {
        &self.value_type
    }

    /// Get the description of the option.
    pub fn description(&self) -> &String {
        &self.description
    }
}
