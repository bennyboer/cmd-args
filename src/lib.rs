mod error;
mod parser;
mod option;
mod group;
mod arg;

pub use parser::Parser;
pub use option::{OptionType, OptionValue, OptionDescriptor};
pub use group::Group;
pub use arg::{ArgType, ArgValue, ArgDescriptor};

// TODO Write unit tests for the library
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
