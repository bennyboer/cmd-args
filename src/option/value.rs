use std::fmt;
use crate::option;

/// Possible values for a CLI option.
pub enum Value {
    Bool { value: bool },
    Str { value: String },
    Int { value: i32 },
    Float { value: f64 },
}

impl Value {
    /// Parse option value from string.
    pub fn parse(option_type: &option::Type, input: &str) -> crate::parser::Result<Value> {
        match option_type {
            option::Type::Bool { default: _ } => Ok(Value::Bool {
                value: input.parse()?
            }),
            option::Type::Str { default: _ } => Ok(Value::Str {
                value: String::from(input)
            }),
            option::Type::Int { default: _ } => Ok(Value::Int {
                value: input.parse()?
            }),
            option::Type::Float { default: _ } => Ok(Value::Float {
                value: input.parse()?
            }),
        }
    }

    /// Create option value from default option value.
    pub fn from_default(option_type: &option::Type) -> Value {
        match option_type {
            option::Type::Bool { default } => Value::Bool {
                value: *default
            },
            option::Type::Str { default } => Value::Str {
                value: default.clone()
            },
            option::Type::Int { default } => Value::Int {
                value: *default
            },
            option::Type::Float { default } => Value::Float {
                value: *default
            },
        }
    }

    /// Get the boolean typed value.
    pub fn bool(&self) -> Option<bool> {
        match self {
            Value::Bool { value } => Some(*value),
            _ => None,
        }
    }

    /// Get the string typed value.
    pub fn str(&self) -> Option<&String> {
        match self {
            Value::Str { value } => Some(value),
            _ => None,
        }
    }

    /// Get the integer typed value.
    pub fn int(&self) -> Option<i32> {
        match self {
            Value::Int { value } => Some(*value),
            _ => None,
        }
    }

    /// Get the float typed value.
    pub fn float(&self) -> Option<f64> {
        match self {
            Value::Float { value } => Some(*value),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Value::Bool { value } => value.to_string(),
            Value::Str { value } => value.to_string(),
            Value::Int { value } => value.to_string(),
            Value::Float { value } => value.to_string(),
        })
    }
}
