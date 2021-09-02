use std::fs;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

// Bot Token
const TOKEN: &str = "";

static mut WORDS: Option<Vec<String>> = None;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore all messages created by bots
        if msg.author.bot {
            return;
        }

        let has_word = has_bad_word(&msg.content);
        if has_word.is_some() {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title("𝐋𝐀𝐍𝐆𝐔𝐀𝐆𝐄!");
                        e.description(&format!(
                            "'{}' is a very bad word!",
                            capitalize(has_word.unwrap())
                        ));
                        e.color(0xE32828);
                        e
                    });
                    m
                })
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }

        println!(
            "({}) [{}:{}]: {}",
            emoji_for_bool(has_word.is_some()),
            msg.author.name,
            msg.author.discriminator,
            msg.content
        );
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{}:{} is ready!", ready.user.name, ready.user.discriminator);
    }
}

#[tokio::main]
async fn main() {
    // Load *words* into a vector
    let word_string: String = fs::read_to_string("data/words.txt").expect("Could not read file");
    let words: Vec<String> = word_string
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.replace('\r', "").to_string())
        .collect();
    unsafe { WORDS = Some(words) };

    let mut client = Client::builder(&TOKEN)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

/// Check if a massage contains a bad word
fn has_bad_word(message: &str) -> Option<&str> {
    let words = unsafe { WORDS.as_ref().unwrap() };
    for word in words {
        if message.contains(word) {
            return Some(word);
        }
    }
    None
}

fn emoji_for_bool(b: bool) -> &'static str {
    if b {
        return "✅";
    }
    "❌"
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
