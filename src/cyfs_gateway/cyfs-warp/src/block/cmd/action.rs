use super::cmd::*;
use crate::block::context::Context;
use std::sync::Arc;

pub struct ActionCommandParser {
    action: CommandAction,
}

impl ActionCommandParser {
    pub fn new(action: CommandAction) -> Self {
        ActionCommandParser { action }
    }
}

impl CommandParser for ActionCommandParser {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String> {
        // Args must be empty
        if !args.trim().is_empty() {
            let msg = format!("Invalid action command: {}", args);
            error!("{}", msg);
            return Err(msg);
        }

        let cmd = ActionCommandExecuter {
            action: self.action.clone(),
        };
        Ok(Arc::new(Box::new(cmd)))
    }
}

// Drop & Pass command
pub struct ActionCommandExecuter {
    action: CommandAction,
}

impl ActionCommandExecuter {
    pub fn new(action: CommandAction) -> Self {
        ActionCommandExecuter { action }
    }
}


#[async_trait::async_trait]
impl CommandExecuter for ActionCommandExecuter {
    async fn exec(&self, _context: &mut Context) -> Result<CommandResult, String> {
        Ok(CommandResult {
            success: true,
            action: self.action.clone(),
            error_code: 0,
        })
    }
}
