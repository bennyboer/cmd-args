mod error;
mod group;
mod help;

pub mod arg;
pub mod option;
pub mod parser;

pub use group::Group;
pub use help::{HelpEntry, HelpPrinter};

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
        let result = parser::parse_from(group, &args[..], None);

        assert!(result.is_ok());
    }

    #[test]
    fn group_alias_test() {
        let group = Group::new(Box::new(|_, _| {
            assert!(false);
        }), "Simple group")
            .add_child("test", Some(vec!("t")), Group::new(Box::new(|_, _| {
                assert!(true);
            }), "Group with aliases"));

        let args: Vec<&str> = vec!("dummy.exe", "t");
        let result = parser::parse_from(group, &args[..], None);

        assert!(result.is_ok());
    }

    // TODO
    // - Test help output
    // - Test nested groups
    // - Test all arg, option types and values
}
