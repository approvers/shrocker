use std::ffi::OsString;

use gethostname::gethostname;

use crate::cmd_exec::CommandExecuteResult;

use super::CommandExecutor;

pub struct LocalCommandExecutor {}

impl LocalCommandExecutor {
    pub fn new() -> Self {
        Self::check_host_is_authorized();

        LocalCommandExecutor {}
    }

    fn check_host_is_authorized() {
        let authorized_hostname = std::env::var("AUTHORIZED_HOST").unwrap();
        let current_hostname = gethostname();

        println!("  Authorized: {authorized_hostname}");
        println!("     Current: {}", current_hostname.to_string_lossy());

        if OsString::from(&authorized_hostname) == current_hostname {
            println!("This machine is authorized to run this core!");
        } else {
            eprintln!("NOT AUTHORIZED HOST TO RUN THIS CORE.");

            std::process::exit(255);
        }
    }
}

impl CommandExecutor for LocalCommandExecutor {
    async fn execute(&self, command_line: &str) -> CommandExecuteResult {
        println!("==> {command_line}");

        let mut cmd = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command_line)
            .spawn()
            .unwrap();

        let result = cmd.wait().await.unwrap();
        println!("   --> was {result}");

        if result.success() {
            Ok(())
        } else {
            Err(result.code().unwrap() as u8)
        }
    }
}
