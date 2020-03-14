use std::fmt;
use crate::arg;

/// Possible argument values.
pub enum Value {
    Bool { value: bool },
    Str { value: String },
    Int { value: i32 },
    Float { value: f64 },
}

impl Value {
    /// Parse argument value from string.
    pub fn parse(arg_type: &arg::Type, input: &str) -> crate::parser::Result<Value> {
        match arg_type {
            arg::Type::Bool => Ok(Value::Bool {
                value: input.parse()?
            }),
            arg::Type::Str => Ok(Value::Str {
                value: String::from(input)
            }),
            arg::Type::Int => Ok(Value::Int {
                value: input.parse()?
            }),
            arg::Type::Float => Ok(Value::Float {
                value: input.parse()?
            }),
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
