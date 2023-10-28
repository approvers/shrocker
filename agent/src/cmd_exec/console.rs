use super::{CommandExecutor, CommandExecuteResult};

pub struct ConsoleCommandExecutor;

impl CommandExecutor for ConsoleCommandExecutor {
    async fn execute(&self, command_line: &str) -> CommandExecuteResult {
        println!("==> {command_line}");

        Ok(())
    }
}