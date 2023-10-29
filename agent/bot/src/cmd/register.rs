use anyhow::Result;
use indoc::{formatdoc, indoc};
use shrocker_agent::{Agent, values::Username, scenario::CreateUserError};

use crate::report::Reporter;

use super::BotContext;

pub async fn perform_register<A, R>(ctx: &mut BotContext<'_, '_, '_, A, R>, user: &str, public_key: &str) -> Result<()>
    where A: Agent, R: Reporter
{
    let ssh_pub_key = public_key.trim();

    let Ok(user) = user.parse::<Username>() else {
        ctx.reporter.error("Username is in the invalid form! Must match to `/[A-Za-z0-2_-]{8,255}/`.", None).await?;
        return Ok(());
    };

    let Ok(key) = shrocker_agent::scenario::validate_key(ctx.agent, ssh_pub_key).await else {
        diagnose_key_error(ctx, ssh_pub_key).await;
        return Ok(());
    };

    ctx.reporter.processing("The key looks okay! Now creating a new user...", None).await.unwrap();
    let user_create_result = shrocker_agent::scenario::create_user(ctx.agent, &user, &key).await;
    if let Err(e) = user_create_result {
        diagnose_user_create_error(ctx, e).await;
        return Ok(());
    }

    let user = user.to_string();
    ctx.reporter.success(&formatdoc!{"
        Okay! Created a new user. Welcome, {user}!
        Now I will DM you to send the connection configuration. Don't lose it, because I won't send this again!
    "}, None).await.unwrap();

    Ok(())
}

async fn diagnose_key_error<A, R>(ctx: &mut BotContext<'_, '_, '_, A, R>, raw_key: &str)
    where A: Agent, R: Reporter
{
    if raw_key.contains("PRIVATE") {
        ctx.reporter.error(indoc! {"
            That doesn't look like public key..?
            At least `ssh-keygen` said \"mmm that's not a public key\".

            This might be rude, but please allow me to inform that you maybe have just send me your *PRIVATE* key.
            If that's true, you'd better to remove the message you just sent, before the things get too interesting..!
        "}, None).await.unwrap();
        return;
    }

    ctx.reporter.error(indoc! {"
        That doesn't look like public key..?
        At least `ssh-keygen` said \"mmm that's not a public key\".
    "}, None).await.unwrap();
}

async fn diagnose_user_create_error<A, R>(ctx: &mut BotContext<'_, '_, '_, A, R>, err: CreateUserError)
    where A: Agent, R: Reporter
{
    match err {
        CreateUserError::AlreadyExists => {
            ctx.reporter.error(indoc! {"
                That's rare, that user already exists!
                Pick an other username.
            "}, None).await.unwrap();
        }
        CreateUserError::UserCreationFail(exit_code) => {
            ctx.reporter.error(&formatdoc! {"
                That's odd, I failed to create a new user... I think this is fault in my side.
                The command to create new user failed with the code `{exit_code}`.
                Please try creating a new user with the other name. But I think you can create a new user with the same name if you really want.
            "}, None).await.unwrap();
        }
        CreateUserError::UserInitFail(exit_code) => {
            ctx.reporter.error(&formatdoc! {"
                This is really bad, I failed to initialize your user! Nobody can log in to the user.
                The command to initialize new user failed with the code `{exit_code}`.
                Sorry about that, but can you create a new user with the other name again?
            "}, None).await.unwrap();
        }
    }
}
