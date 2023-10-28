use super::{CommandExecutor, CommandExecuteResult};

pub struct ConsoleCommandExecutor;

impl CommandExecutor for ConsoleCommandExecutor {
    async unsafe fn execute(command_line: String) -> CommandExecuteResult {
        println!("==> {command_line}");

        CommandExecuteResult { exit_code: 0 }
    }
}