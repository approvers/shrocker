mod cmd;
mod config;
mod report;
mod handler;

use std::{env, path::Path};

use config::Configuration;
use handler::Handler;

use serenity::{prelude::GatewayIntents, Client};

use shrocker_agent::initialize_core;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let agent = initialize_core();

    let token = env::var("DISCORD_TOKEN").expect("should be specified");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_PRESENCES;

    let handler = Handler {
        agent,
        config: Configuration {
            connection_guidance: load_connection_file(Path::new(&env::var("CONNECTION_GUIDANCE").unwrap()))
        },
    };

    let mut client = Client::builder(token, intents)
        .event_handler(handler)
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

fn load_connection_file(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}
