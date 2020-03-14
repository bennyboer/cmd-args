use std::fmt;

/// Possible types for a CLI option.
pub enum Type {
    Bool { default: bool },
    Str { default: String },
    Int { default: i32 },
    Float { default: f64 },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Type::Bool { default: _ } => "boolean",
            Type::Str { default: _ } => "string",
            Type::Int { default: _ } => "integer",
            Type::Float { default: _ } => "float",
        })
    }
}
