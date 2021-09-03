use serenity::{model::channel::Message, prelude::*};
use super::super::common::check_send;

pub async fn message(ctx: Context, msg: Message) {
    if msg.author.bot {
        return;
    }
    check_send(
        ctx,
        msg.content,
        msg.author.name,
        msg.author.discriminator,
        msg.channel_id,
    )
    .await;
}
