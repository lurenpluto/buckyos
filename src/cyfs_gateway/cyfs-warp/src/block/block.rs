
use std::collections::HashMap;

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

// Command or Expression
#[derive(Debug, Clone)]
pub enum Expression {
    Command(Command),
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
