use super::context::Context;
use regex::Regex;
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

// http-sni-probe command
pub struct HttpSniProbeCommandExecuter {}

impl HttpSniProbeCommandExecuter {
    pub fn new() -> Self {
        HttpSniProbeCommandExecuter {}
    }
}

impl CommandParser for HttpSniProbeCommandExecuter {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String> {
        // Args must be empty
        if !args.trim().is_empty() {
            let msg = format!("Invalid http-sni-probe command: {}", args);
            error!("{}", msg);
            return Err(msg);
        }

        let cmd = HttpSniProbeCommandExecuter::new();
        Ok(Arc::new(Box::new(cmd)))
    }
}

#[async_trait::async_trait]
impl CommandExecuter for HttpSniProbeCommandExecuter {
    async fn exec(&self, context: &mut Context) -> Result<CommandResult, String> {
        todo!("http-sni-probe not implemented yet");

        context.set_value("REQ_HEADER.host", "");

        Ok(CommandResult::success())
    }
}

// Match command, like: match REQ_HEADER.host "*.local"
pub struct MatchCommandExecuter {
    pub key: String,
    pub pattern: Regex,
}

impl CommandParser for MatchCommandExecuter {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String> {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid match command".to_string());
        }

        let key = parts[0].to_string();
        let pattern = Regex::new(parts[1]).map_err(|e| format!("Invalid pattern: {}", e))?;

        let cmd = MatchCommandExecuter { key, pattern };
        Ok(Arc::new(Box::new(cmd)))
    }
}

#[async_trait::async_trait]
impl CommandExecuter for MatchCommandExecuter {
    async fn exec(&self, context: &mut Context) -> Result<CommandResult, String> {
        // First get the value
        let value = context.get_value(&self.key);
        if value.is_none() {
            return Ok(CommandResult::failure(1));
        }

        // Then match the value
        let value = value.unwrap();
        if self.pattern.is_match(&value) {
            Ok(CommandResult::success())
        } else {
            Ok(CommandResult::failure(2))
        }
    }
}

// EXEC command, like: EXEC app1
pub struct ExternalCommandExecuter {
    pub command: String,
}

impl CommandParser for ExternalCommandExecuter {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String> {
        // Args should not be empty
        if args.trim().is_empty() {
            let msg = format!("Invalid exec command: {}", args);
            error!("{}", msg);
            return Err(msg);
        }

        let cmd = ExternalCommandExecuter {
            command: args.to_string(),
        };
        Ok(Arc::new(Box::new(cmd)))
    }
}

#[async_trait::async_trait]
impl CommandExecuter for ExternalCommandExecuter {
    async fn exec(&self, context: &mut Context) -> Result<CommandResult, String> {
        todo!("exec command not implemented yet");

        Ok(CommandResult::success())
    }
}

// CMD: set_lable_by_host_db REQ_HEADER.host
pub struct SetLabelByHostDbCommandExecuter {
    pub key: String,
}

impl SetLabelByHostDbCommandExecuter {
    pub fn new(key: &str) -> Self {
        SetLabelByHostDbCommandExecuter {
            key: key.to_string(),
        }
    }
}

impl CommandParser for SetLabelByHostDbCommandExecuter {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String> {
        let args = args.trim();
        if args.is_empty() {
            let msg = format!("Invalid set_label_by_host_db command: {}", args);
            error!("{}", msg);
            return Err(msg);
        }

        // Only accept one argument
        if args.split_whitespace().count() != 1 {
            let msg = format!("Invalid set_label_by_host_db command: {}", args);
            error!("{}", msg);
            return Err(msg);
        }

        let cmd = SetLabelByHostDbCommandExecuter::new(args);
        Ok(Arc::new(Box::new(cmd)))
    }
}

#[async_trait::async_trait]
impl CommandExecuter for SetLabelByHostDbCommandExecuter {
    async fn exec(&self, context: &mut Context) -> Result<CommandResult, String> {
        // First get value from context, then load label from host db, then set label to request header
        let value = context.get_value(self.key.as_str());
        if value.is_none() {
            return Ok(CommandResult::failure(1));
        }

        context.set_label_from_host_db(&value.unwrap());

        Ok(CommandResult::success())
    }
}

// CMD: have_label REQ_HEADER.label "xxx"
pub struct HaveLabelCommandExecuter {
    pub key: String,
    pub label: String,
}

impl HaveLabelCommandExecuter {
    pub fn new(key: &str, label: &str) -> Self {
        HaveLabelCommandExecuter {
            key: key.to_string(),
            label: label.to_string(),
        }
    }
}

impl CommandParser for HaveLabelCommandExecuter {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String> {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.len() != 2 {
            let msg = format!("Invalid have_label command: {}", args);
            error!("{}", msg);
            return Err(msg);
        }

        let cmd = HaveLabelCommandExecuter::new(parts[0], parts[1]);
        Ok(Arc::new(Box::new(cmd)))
    }
}

#[async_trait::async_trait]
impl CommandExecuter for HaveLabelCommandExecuter {
    async fn exec(&self, context: &mut Context) -> Result<CommandResult, String> {
        // First get value from context, then check if the label exists
        let labels = context.get_value(self.key.as_str());
        if labels.is_none() {
            return Ok(CommandResult::failure(1));
        }

        // Parse the labels
        let labels = labels.unwrap();
        let labels: Vec<&str> = labels.split(',').collect();
        for label in labels {
            if label == self.label {
                return Ok(CommandResult::success());
            }
        }

        Ok(CommandResult::failure(2))
    }
}
