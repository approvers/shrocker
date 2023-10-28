mod register;
mod report;

use std::env;

use indoc::indoc;
use serenity::framework::standard::{CommandResult, Args};
use serenity::framework::standard::macros::{command, group};
use serenity::model::prelude::{UserId, Message};
use serenity::prelude::Context;
use serenity::{framework::StandardFramework, prelude::GatewayIntents, Client};

use crate::register::perform_register;
use crate::report::{SerenityReporter, Reporter, ReportPreference};

#[group]
#[commands(register)]
struct RegisterCommand;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let user_id = env::var("DISCORD_USER_ID")
        .ok()
        .map(|x| x.parse::<UserId>().expect("The format of DISCORD_USER_ID is not valid Snowflake"));
    let token = env::var("DISCORD_TOKEN").expect("should be specified");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_PRESENCES;

    let fw = StandardFramework::new()
        .configure(|c| c.on_mention(user_id).prefix("!shr "))
        .group(&REGISTERCOMMAND_GROUP);

    let mut client = Client::builder(token, intents)
        .framework(fw)
        .await
        .expect("Client could not be created");

    match client.start().await {
        Ok(()) => {
            println!("Agent bot is shuted down");
        }
        Err(reason) => {
            eprintln!("[!] There was a error and agent bot was shuted down");
            eprintln!("{reason}")
        }
    }
}

#[command]
async fn register(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut reporter = SerenityReporter::new(ctx, msg);

    args.quoted();
    if args.len() != 2 {
        reporter.report(
            indoc! {"
            🤷 引数が足りないか、多すぎます!
            この形式で入力してください: `'<新しいユーザ名>'` `'<SSH 公開鍵>'`
            正しい形式で入力しているのにこのエラーが出る場合は、引数を `'` や `\"` で囲ってみて下さい。
            "}, None).await?;
        return Ok(());
    }

    let user = args.single::<String>()?;
    let ssh_key = args.single::<String>()?;

    reporter.report(&format!("📝 `{user}` として新しいユーザを作成しています……"), None).await?;

    perform_register(&user, &ssh_key, &mut reporter).await?;

    reporter.report(indoc! {"
        ✅ 作成しました!
        サーバへの接続情報をお送りします。1 度しか送られないので、大切に保管して下さい!
    "}, None).await?;

    let dm_result = msg.author.direct_message(ctx, |msg| {
        msg.content("foobar")
    }).await;

    if dm_result.is_err() {
        reporter.report(indoc! {"
        🛑 DM を送信できませんでした!
        DM が許可されていないかもしれません。DM を許可した上で再度ユーザを別の名前で作り直すか、Shrocker を運営している人に接続情報を教えてもらってください。
        "}, ReportPreference::EXPLICIT_NEW_MESSAGE).await?;
        dm_result?;
    }

    Ok(())
}
