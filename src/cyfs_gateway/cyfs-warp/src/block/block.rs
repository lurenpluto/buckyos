use super::cmd::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    And,  // &&
    Or,   // ||
    None, // None of the above
}

// Single command
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Clone)]
pub struct CommandItem {
    pub command: Command,
    pub executor: Option<CommandExecutorRef>,
}

impl fmt::Debug for CommandItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use command instead of self
        write!(f, "{:?}", self.command)
    }
}

impl CommandItem {
    pub fn new(name: String, args: Vec<String>) -> Self {
        let command = Command { name, args };
        Self {
            command,
            executor: None,
        }
    }

    pub fn new_empty() -> Self {
        Self::new("".to_string(), Vec::new())
    }
}

// Command or Expression
#[derive(Debug, Clone)]
pub enum Expression {
    Command(CommandItem),
    Group(Vec<(Expression, Operator)>), // Sub-expression in brackets
    Goto(String),                       // Goto label
}

// Line of commands, top level structure
#[derive(Debug)]
pub struct Line {
    pub label: Option<String>, // Label of the line
    pub expressions: Vec<(Expression, Operator)>,
}

// Block of lines
#[derive(Debug)]
pub struct Block {
    pub lines: Vec<Line>,
    pub label_map: HashMap<String, usize>, // Label to line index
}

impl Block {
    pub fn new() -> Self {
        Block {
            lines: Vec::new(),
            label_map: HashMap::new(),
        }
    }
}
