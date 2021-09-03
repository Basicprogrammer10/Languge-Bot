use serenity::{model, prelude::*};

static mut WORDS: Option<Vec<String>> = None;

/// Check if a message contains a bad word then send a message to the channel
pub async fn check_send(
    ctx: Context,
    content: String,
    author_name: String,
    author_tag: u16,
    channel_id: model::id::ChannelId,
) {
    let has_word = has_bad_word(&content);
    if has_word.is_some() {
        if let Err(why) = channel_id
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
        author_name,
        author_tag,
        content
    );
}

pub fn set_words(words: Vec<String>) {
    unsafe {
        WORDS = Some(words);
    }
}

/// Check if a massage contains a bad word
pub fn has_bad_word(message: &str) -> Option<&str> {
    let words = unsafe { WORDS.as_ref().unwrap() };

    for word in words {
        if message.contains(word) {
            return Some(word);
        }
    }
    None
}

pub fn emoji_for_bool(b: bool) -> &'static str {
    if b {
        return "✅";
    }
    "❌"
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
