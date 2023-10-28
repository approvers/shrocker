use crate::cmd_exec::CommandExecutor;

pub struct Context<X>
    where X: CommandExecutor
{
    pub(crate) cmd_exec: X,
}
