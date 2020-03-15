use std::rc::Rc;
use crate::{option, arg};
use std::collections::HashMap;

/// Consumer for the parsed result (arguments and options).
type ParserResultConsumer = Box<dyn Fn(&Vec<arg::Value>, &HashMap<&str, option::Value>)>;

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
    options: Option<HashMap<Rc<String>, Rc<option::Descriptor>>>,

    /// Descriptors for all anticipated arguments.
    arguments: Vec<arg::Descriptor>,

    /// Child groups.
    children: HashMap<Rc<String>, Rc<Group>>,

    /// Lookup of child groups by known aliases (including name).
    children_lookup: HashMap<Rc<String>, Rc<Group>>,

    /// Lookup of aliases by group name.
    alias_lookup: HashMap<Rc<String>, Vec<Rc<String>>>,

    /// Consumer called with the parsed options and arguments for this group.
    consumer: ParserResultConsumer,

    /// Description of the group.
    description: String,
}

impl Group {
    /// Create new group configuration.
    pub fn new(consumer: ParserResultConsumer, description: &str) -> Self {
        Group {
            options: Some(HashMap::new()),
            arguments: Vec::new(),
            children: HashMap::new(),
            children_lookup: HashMap::new(),
            alias_lookup: HashMap::new(),
            consumer,
            description: String::from(description),
        }
    }

    /// Add an argument to this group.
    pub fn add_argument(mut self, argument: arg::Descriptor) -> Self {
        self.arguments.push(argument);

        self
    }

    /// Get argument descriptors.
    pub fn get_arguments(&self) -> &Vec<arg::Descriptor> {
        &self.arguments
    }

    /// Add an option to this group.
    pub fn add_option(mut self, option: option::Descriptor) -> Self {
        assert!(!&self.options.as_ref().unwrap().contains_key(option.name()));

        self.options.as_mut().unwrap().insert(option.take_name(), Rc::new(option));

        self
    }

    /// Take ownership of all specified options.
    pub fn get_options(&self) -> &HashMap<Rc<String>, Rc<option::Descriptor>> {
        self.options.as_ref().unwrap()
    }

    /// Add a child group known by the passed name.
    pub fn add_child(mut self, name: &str, aliases: Option<Vec<&str>>, group: Group) -> Self {
        let name = Rc::new(String::from(name));
        let group = Rc::new(group);

        assert!(!self.children.contains_key(&name));
        self.children.insert(Rc::clone(&name), Rc::clone(&group));

        // Insert aliases in lookup
        assert!(!self.children_lookup.contains_key(&name));
        self.children_lookup.insert(Rc::clone(&name), Rc::clone(&group));

        if aliases.is_some() {
            let aliases = aliases.unwrap();
            let mut alias_vec = Vec::with_capacity(aliases.len());
            for alias in aliases {
                let alias = Rc::new(String::from(alias));
                alias_vec.push(Rc::clone(&alias));

                assert!(!self.children_lookup.contains_key(&alias));
                self.children_lookup.insert(Rc::clone(&alias), Rc::clone(&group));
            }

            self.alias_lookup.insert(Rc::clone(&name), alias_vec);
        }

        self
    }

    /// Get known aliases for the passed group name.
    pub fn get_aliases_for_group_name(&self, group_name: &String) -> Option<&Vec<Rc<String>>> {
        self.alias_lookup.get(group_name)
    }

    /// Get children of the group.
    pub fn get_children(&self) -> &HashMap<Rc<String>, Rc<Group>> {
        &self.children
    }

    /// Get a child known for the passed alias (including name).
    pub fn get_child_known_for(&self, alias: &str) -> Option<Rc<Group>> {
        match self.children_lookup.get(&String::from(alias)) {
            Some(v) => Some(Rc::clone(v)),
            None => None
        }
    }

    /// Get the registered function to consume the parsed arguments and options.
    pub fn get_consumer(&self) -> &ParserResultConsumer {
        &self.consumer
    }

    /// Get the description of the group.
    pub fn description(&self) -> &String {
        &self.description
    }
}
