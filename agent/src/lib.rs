mod cmd_exec;
mod scenario;
mod context;
pub mod values;

use cmd_exec::{initialize_executor, CommandExecutor};
use values::Username;

pub fn initialize_core() -> impl Agent {
    AgentImpl {
        ctx: context::Context {
            cmd_exec: initialize_executor()
        }
    }
}

pub trait Agent {
    fn create_user(&self, username: &Username, ssh_public_key: &str) -> impl std::future::Future<Output = ()> + Send;
}

pub struct AgentImpl<X>
    where X: CommandExecutor + Send + Sync
{
    ctx: context::Context<X>
}

impl<X> Agent for AgentImpl<X>
    where Self: Send + Sync, X: CommandExecutor + Send + Sync
{
    async fn create_user(&self, username: &Username, ssh_public_key: &str) {
        scenario::create_user::create_user(&self.ctx, username, ssh_public_key).await;
    }
}

