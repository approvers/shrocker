use serenity::{prelude::Context, model::prelude::Message};
use shrocker_agent::Agent;

use crate::report::Reporter;

pub mod register;

pub struct BotContext<'agent, 'ctx, 'reporter, A: Agent, R: Reporter> {
    pub agent: &'agent A,
    pub ctx: &'ctx Context,
    pub new_message: &'ctx Message,
    pub reporter: &'reporter mut R,
}
