use super::cmd::*;
use crate::block::context::Context;
use std::sync::Arc;

// CMD: set_lable_by_host_db REQ_HEADER.host
pub struct SetLabelByHostDbCommandParser {}

impl SetLabelByHostDbCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandParser for SetLabelByHostDbCommandParser {
    fn parse(&self, args: &str) -> Result<CommandExecuterRef, String> {
        // Args should not be empty
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

// Set label by host db command executer
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
pub struct HaveLabelCommandParser {

}

impl HaveLabelCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandParser for HaveLabelCommandParser {
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
