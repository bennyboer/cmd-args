mod error;
mod group;

pub mod arg;
pub mod option;
pub mod parser;

pub use group::Group;

#[cfg(test)]
mod tests {
    use crate::{Group, option, arg, parser};

    #[test]
    fn simple() {
        let group = Group::new(Box::new(|args, options| {
            let test = args[0].str().unwrap();
            assert_eq!(test, "I am a test text!");

            let the_truth = options.get("the-truth").unwrap().int().unwrap();
            assert_eq!(the_truth, 42);
        }), "Simple group without nested sub-commands")
            .add_option(option::Descriptor::new("the-truth", option::Type::Int { default: 42 }, "The truth about everything"))
            .add_argument(arg::Descriptor::new(arg::Type::Str, "Test text"));

        let args: Vec<&str> = vec!("dummy.exe", "I am a test text!");
        let result = parser::parse(group, &args[..]);

        assert!(result.is_ok());
    }

    // TODO
    // - Test help output
    // - Test nested groups
    // - Test all arg, option types and values
}
