use std::fmt;

/// Possible argument types.
pub enum Type {
    Bool,
    Str,
    Int,
    Float,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Type::Bool => "boolean",
            Type::Str => "string",
            Type::Int => "integer",
            Type::Float => "float",
        })
    }
}
