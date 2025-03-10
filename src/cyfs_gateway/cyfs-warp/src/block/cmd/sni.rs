use super::cmd::*;
use crate::block::context::Context;
use std::sync::Arc;

// http-sni-probe command parser
pub struct HttpSniProbeCommandParser {}

impl HttpSniProbeCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandParser for HttpSniProbeCommandParser {
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

// http-sni-probe command executer
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
