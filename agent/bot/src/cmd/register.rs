use anyhow::Result;
use indoc::formatdoc;
use shrocker_agent::{Agent, values::Username};

use crate::report::Reporter;

use super::BotContext;

pub async fn perform_register<A, R>(ctx: &mut BotContext<'_, '_, '_, A, R>, user: &str, public_key: &str) -> Result<()>
    where A: Agent, R: Reporter
{
    let Ok(user) = user.parse::<Username>() else {
        ctx.reporter.error("Username is in the invalid form! Must match to `/[A-Za-z0-2_-]{8,255}/`.", None).await?;
        return Ok(());
    };

    ctx.reporter.processing("Creating a new user...", None).await.unwrap();

    ctx.agent.create_user(&user, public_key).await;

    let user = user.to_string();
    ctx.reporter.success(&formatdoc!{"
        Okay! Created a new user. Welcome, {user}!
        Now I will DM you to send the connection configuration. Don't lose it, because I won't send this again!
    "}, None).await.unwrap();

    Ok(())
}