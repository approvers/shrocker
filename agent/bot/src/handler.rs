use clap::{Parser, Args};
use indoc::indoc;
use serenity::{prelude::*, model::prelude::*, async_trait};
use shrocker_agent::Agent;

use crate::{report::{SerenityReporter, Reporter}, cmd::{register::perform_register, BotContext}, config::Configuration};

const BOT_PREFIX: &str = "!shr";

pub struct Handler<A>
    where A: Agent
{
    pub agent: A,
    pub config: Configuration,
}

#[async_trait]
impl<A> EventHandler for Handler<A>
    where A: Agent + Send + Sync
{
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("✓ Discord Bot is running");
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.author.bot {
            return;
        }

        let content = new_message.content.to_owned();
        if !content.starts_with(BOT_PREFIX) {
            return;
        }

        let mut reporter = SerenityReporter::new(&ctx, &new_message);

        let Some(segments) = shlex::split(&content) else {
            reporter.error("Couldn't parse the input. Maybe not closed quotations?", None).await.unwrap();
            return
        };

        let parsed = match BotCommand::try_parse_from(segments) {
            Ok(parsed) => parsed,
            Err(err) => {
                let error = err.render().to_string();
                reporter.report(&format!("```{error}```"), None).await.unwrap();
                return;
            }
        };

        let mut bot_ctx = BotContext {
            agent: &self.agent,
            ctx: &ctx,
            new_message: &new_message,
            reporter: &mut reporter,
            config: &self.config,
        };

        let cmd_result = match parsed {
            BotCommand::Register(args) => {
                perform_register(&mut bot_ctx, &args.user, &args.ssh_public_key).await
            }
        };

        let Err(err) = cmd_result else { return; };

        reporter.error(indoc! {"
            There was a error during execution.
            The log might contain sensitive data so I will not show it here.

            <@599423913877045258> Check out the Shrocker server m8 it might be toast now
        "}, None).await.unwrap();
        eprintln!("===== ERROR =====\n{err}");
    }
}

#[derive(Debug, Parser)]
#[command(name = BOT_PREFIX)]
#[command(bin_name = BOT_PREFIX)]
enum BotCommand {
    /// Create a new user on Shrocker Shared Server
    Register(RegisterArgs)
}

#[derive(Args, Debug)]
struct RegisterArgs {
    /// Newly created user's name.
    /// Must be in the form of /[A-Za-z0-9_-]{2,255}/
    user: String,

    /// SSH *PUBLIC* Key.
    ssh_public_key: String
}
