use super::cmd::*;
use crate::block::context::Context;
use std::sync::Arc;


// EXEC command, like: EXEC app1
pub struct ExternalCommandParser {}

impl CommandParser for ExternalCommandParser {
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

// EXEC command executer
pub struct ExternalCommandExecuter {
    pub command: String,
}

#[async_trait::async_trait]
impl CommandExecuter for ExternalCommandExecuter {
    async fn exec(&self, context: &mut Context) -> Result<CommandResult, String> {
        todo!("exec command not implemented yet");

        Ok(CommandResult::success())
    }
}
