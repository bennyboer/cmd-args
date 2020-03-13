use std::fmt;

/// Possible argument types.
pub enum ArgType {
    Bool,
    Str,
    Int,
    Float,
}

impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ArgType::Bool => "boolean",
            ArgType::Str => "string",
            ArgType::Int => "integer",
            ArgType::Float => "float",
        })
    }
}

/// Possible argument values.
pub enum ArgValue {
    Bool { value: bool },
    Str { value: String },
    Int { value: i32 },
    Float { value: f64 },
}

impl ArgValue {
    /// Parse argument value from string.
    pub fn parse(arg_type: &ArgType, input: &str) -> super::parser::Result<ArgValue> {
        match arg_type {
            ArgType::Bool => Ok(ArgValue::Bool {
                value: input.parse()?
            }),
            ArgType::Str => Ok(ArgValue::Str {
                value: String::from(input)
            }),
            ArgType::Int => Ok(ArgValue::Int {
                value: input.parse()?
            }),
            ArgType::Float => Ok(ArgValue::Float {
                value: input.parse()?
            }),
        }
    }
}

impl fmt::Display for ArgValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ArgValue::Bool { value } => value.to_string(),
            ArgValue::Str { value } => value.to_string(),
            ArgValue::Int { value } => value.to_string(),
            ArgValue::Float { value } => value.to_string(),
        })
    }
}

pub struct ArgDescriptor {
    value_type: ArgType,
    description: String,
}

impl ArgDescriptor {
    /// Create a new argument descriptor.
    pub fn new(value_type: ArgType, description: &str) -> Self {
        ArgDescriptor {
            value_type,
            description: String::from(description),
        }
    }

    /// Get the type of the argument value.
    pub fn value_type(&self) -> &ArgType {
        &self.value_type
    }

    /// Get the description of the argument.
    pub fn description(&self) -> &String {
        &self.description
    }
}
