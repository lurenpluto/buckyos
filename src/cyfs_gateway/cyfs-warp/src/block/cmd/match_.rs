use super::cmd::*;
use crate::block::context::Context;
use regex::Regex;
use std::sync::Arc;


// Match command, like: match REQ_HEADER.host "*.local"

pub struct MatchCommandParser {}

impl MatchCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandParser for MatchCommandParser {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String> {
        let parts: Vec<&str> = args.split_whitespace().collect();
        if parts.len() != 2 {
            let msg = format!("Invalid match command: {}", args);
            error!("{}", msg);
            return Err(msg);
        }

        let key = parts[0].to_string();
        let pattern = Regex::new(parts[1]).map_err(|e| format!("Invalid pattern: {}", e))?;

        let cmd = MatchCommandExecuter { key, pattern };
        Ok(Arc::new(Box::new(cmd)))
    }
}

// Match command executer
pub struct MatchCommandExecuter {
    pub key: String,
    pub pattern: Regex,
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
