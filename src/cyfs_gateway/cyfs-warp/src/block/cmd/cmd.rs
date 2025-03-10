use crate::block::context::Context;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub type CommandParserRef = Arc<Box<dyn CommandParser>>;

pub trait CommandParser {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String>;
}

#[derive(Clone)]
pub struct CommandParserFactory {
    parsers: Arc<Mutex<HashMap<String, CommandParserRef>>>,
}

impl CommandParserFactory {
    pub fn new() -> Self {
        Self {
            parsers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, name: &str, parser: CommandParserRef) {
        let mut parsers = self.parsers.lock().unwrap();
        if let Some(_prev) = parsers.insert(name.to_string(), parser) {
            error!("Command parser {} already exists, will be replaced", name);
        }
    }

    pub fn get_parser(&self, name: &str) -> Option<CommandParserRef> {
        let parsers = self.parsers.lock().unwrap();
        parsers.get(name).cloned()
    }

    pub fn parse(&self, name: &str, args: &str) -> Result<CommandExecuterRef, String> {
        let parser = self.get_parser(name);
        if parser.is_none() {
            let msg = format!("Command parser {} not found", name);
            error!("{}", msg);
            return Err(msg);
        }

        parser.unwrap().parse(args)
    }
}

#[derive(Debug, Clone)]
pub enum CommandAction {
    Ok,
    Drop,
    Pass,
    Goto(String),
}

// The result of a command execution
#[derive(Debug)]
pub struct CommandResult {
    pub success: bool,
    pub action: CommandAction,
    pub error_code: i32,
}

impl CommandResult {
    pub fn success() -> Self {
        CommandResult {
            success: true,
            action: CommandAction::Ok,
            error_code: 0,
        }
    }

    pub fn failure(code: i32) -> Self {
        CommandResult {
            success: false,
            action: CommandAction::Ok,
            error_code: code,
        }
    }

    pub fn drop() -> Self {
        CommandResult {
            success: true,
            action: CommandAction::Drop,
            error_code: 0,
        }
    }

    pub fn pass() -> Self {
        CommandResult {
            success: true,
            action: CommandAction::Pass,
            error_code: 0,
        }
    }

    pub fn goto(target: impl Into<String>) -> Self {
        CommandResult {
            success: true,
            action: CommandAction::Goto(target.into()),
            error_code: 0,
        }
    }

    pub fn is_special_action(&self) -> bool {
        match self.action {
            CommandAction::Ok => false,
            _ => true,
        }
    }
}

// CommandExecuter is the trait for executing a command
#[async_trait::async_trait]
pub trait CommandExecuter: Send + Sync {
    async fn exec(&self, context: &mut Context) -> Result<CommandResult, String>;
}

pub type CommandExecuterRef = Arc<Box<dyn CommandExecuter>>;




