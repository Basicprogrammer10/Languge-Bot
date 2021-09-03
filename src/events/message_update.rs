use super::super::common::check_send;
use serenity::{model::event::MessageUpdateEvent, prelude::*};

pub async fn message_update(ctx: Context, event: MessageUpdateEvent) {
    let content = match event.content {
        Some(m) => m,
        None => "".to_string(),
    };

    let author = event.author.unwrap_or_default();
    check_send(
        ctx,
        content,
        author.name,
        author.discriminator,
        event.channel_id,
    )
    .await;
}
