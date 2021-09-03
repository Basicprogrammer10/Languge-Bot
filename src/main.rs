use std::fs;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, prelude::*},
    prelude::*,
};

mod events;
mod common;
use common::*;

// Fallback Bot Token
const TOKEN: &str = "";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // On User Send Message
    async fn message(&self, ctx: Context, msg: Message) {
        events::message(ctx, msg).await;
    }

    // On User Edit Message
    async fn message_update(
        &self,
        ctx: Context,
        _: Option<Message>,
        _: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        events::message_update(ctx, event).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{}:{} is ready!\n", ready.user.name, ready.user.discriminator);
    }
}

#[tokio::main]
async fn main() {
    // Use first argument as bot token if present
    let token = env::args().nth(1).unwrap_or(TOKEN.to_string());
    if token == "" {
        panic!("No token provided! Use ./languge_bot <TOKEN>");
    }

    // Load *words* into a vector
    let word_string: String = fs::read_to_string("data/words.txt").expect("Could not read file");
    let words: Vec<String> = word_string
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.replace('\r', "").to_string())
        .collect();
    set_words(words);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}