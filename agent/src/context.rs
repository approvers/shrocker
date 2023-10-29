use crate::cmd_exec::CommandExecutor;

pub struct Context<X: CommandExecutor> {
    pub(crate) cmd_exec: X,
}

pub trait HasContext<X: CommandExecutor> {
    fn ctx(&self) -> &Context<X>;
}
