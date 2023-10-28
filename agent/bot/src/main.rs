mod cmd;
mod report;
mod handler;

use std::env;

use handler::Handler;

use serenity::{prelude::GatewayIntents, Client};

use shrocker_agent::initialize_core;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let agent = initialize_core();

    let token = env::var("DISCORD_TOKEN").expect("should be specified");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_PRESENCES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler { agent })
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
