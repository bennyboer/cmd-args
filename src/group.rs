use std::collections::HashMap;
use std::rc::Rc;
use crate::{ArgValue, OptionValue, OptionDescriptor, ArgDescriptor};

type ParserResultConsumer = Box<dyn FnOnce(Vec<ArgValue>, HashMap<String, OptionValue>)>;

/// A group is a collection of possible CLI options and arguments.
/// Essentially it provides the context of a action called via CLI.
/// For example: When calling something like `dummy.exe test --flag`
/// then `test` is the command context since the action `test` should be invoked.
/// Now `--flag` can now be either created on the `Root` group or
/// the `test` group.
/// When options are defined on the root group, they are available to every
/// child group as well, while options defined on a child group are only available to the
/// child group and its children.
pub struct Group {
    /// Descriptors for all anticipated options.
    options: Option<HashMap<Rc<String>, Rc<OptionDescriptor>>>,

    /// Descriptors for all anticipated arguments.
    arguments: Option<Vec<ArgDescriptor>>,

    /// Child groups.
    children: HashMap<String, Group>,

    /// Consumer called with the parsed options and arguments for this group.
    consumer: Option<ParserResultConsumer>,

    /// Description of the group.
    description: String,
}

impl Group {
    /// Create new group configuration.
    pub fn new(consumer: ParserResultConsumer, description: &str) -> Self {
        Group {
            options: Some(HashMap::new()),
            arguments: Some(Vec::new()),
            children: HashMap::new(),
            consumer: Some(consumer),
            description: String::from(description),
        }
    }

    /// Add an argument to this group.
    pub fn add_argument(mut self, argument: ArgDescriptor) -> Self {
        self.arguments.as_mut().unwrap().push(argument);

        self
    }

    /// Take ownership for all specified arguments.
    pub fn take_arguments(&mut self) -> Vec<ArgDescriptor> {
        match self.arguments.take() {
            Some(v) => v,
            None => panic!("Argument list has already moved out")
        }
    }

    /// Add an option to this group.
    pub fn add_option(mut self, option: OptionDescriptor) -> Self {
        assert!(!&self.options.as_ref().unwrap().contains_key(option.name()));

        self.options.as_mut().unwrap().insert(option.take_name(), Rc::new(option));

        self
    }

    /// Take ownership of all specified options.
    pub fn get_options(&mut self) -> &HashMap<Rc<String>, Rc<OptionDescriptor>> {
        self.options.as_ref().unwrap()
    }

    /// Add a child group known by the passed name.
    pub fn add_child(mut self, name: &str, group: Group) -> Self {
        let name = String::from(name);
        assert!(!self.children.contains_key(&name));

        self.children.insert(name, group);

        self
    }

    /// Get children of the group.
    pub fn get_children(&self) -> &HashMap<String, Group> {
        &self.children
    }

    /// Take a child group.
    pub fn get_mut_child(&mut self, name: &str) -> &mut Group {
        self.children.get_mut(name).unwrap()
    }

    /// Check whether the group contains a sub group with the given name.
    pub fn has_child_name(&self, name: &str) -> bool {
        self.children.contains_key(name)
    }

    /// Call the registered function to consume the parsed arguments and options.
    pub fn call_consumer(&mut self, args: Vec<ArgValue>, options: HashMap<String, OptionValue>) {
        self.consumer.take().unwrap()(args, options);
    }

    /// Get the description of the group.
    pub fn description(&self) -> &String {
        &self.description
    }
}
