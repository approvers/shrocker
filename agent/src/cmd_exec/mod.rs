#[cfg(feature = "local")]
mod local;

#[cfg(feature = "console")]
mod console;

#[cfg(all(feature = "local", feature = "console"))]
compile_error!("local and console is specified at the both time - this is illegal!");

#[cfg(not(any(feature = "local", feature = "console")))]
compile_error!("Neither local nor console is specified - this is illegal!");

use std::future::Future;

pub struct CommandExecuteResult {
    pub exit_code: u8,
}

pub trait CommandExecutor {
    /// Runs command on the shared machine.
    ///
    /// ### Safety
    /// The method execute is *NOT* expected to sanitize the `command_line` argument.
    /// The user of execute MUST validate, sanitize the `command_line` argument before executing it!
    ///
    /// These are the nasty inputs that must be considered:
    ///    - `'`, `"`, and back-quote (breaks `'{command_line}'`, `"{command_line}"`)
    ///    - `|`, `>` (might do some unexpected command execution)
    ///    - `../`, `/` (directory traversal!)
    unsafe fn execute(command_line: String) -> impl Future<Output = CommandExecuteResult> + Send;
}
