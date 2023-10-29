mod cmd_exec;
mod context;
pub mod scenario;
pub mod values;

use cmd_exec::{initialize_executor, CommandExecutor};
use context::HasContext;

pub fn initialize_core() -> impl Agent {
    AgentImpl {
        ctx: context::Context {
            cmd_exec: initialize_executor()
        }
    }
}

// hides nasty generics stuff
pub trait Agent: HasContext<Self::Executor> {
    type Executor: CommandExecutor;
}

pub struct AgentImpl<X>
    where X: CommandExecutor,
{
    ctx: context::Context<X>
}

impl<X> HasContext<X> for AgentImpl<X>
    where X: CommandExecutor
{
    fn ctx(&self) -> &context::Context<X> {
        &self.ctx
    }
}

impl<X> Agent for AgentImpl<X> where X: CommandExecutor + Send + Sync {
    type Executor = X;
}
