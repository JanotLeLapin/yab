use std::{env, error::Error};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("DISCORD_TOKEN")?;
    let intents = Intents::GUILD_MESSAGES
        | Intents::GUILD_MEMBERS
        | Intents::GUILD_MESSAGE_TYPING
        | Intents::MESSAGE_CONTENT;
    let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);
    let client = Client::new(token);

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            println!("could not receive event");
            continue;
        };
        match event {
            Event::MessageCreate(msg) => {
                println!("{}: {}", msg.author.name, msg.content);
                if msg.author.bot {
                    continue;
                }

                client
                    .create_message(msg.channel_id)
                    .content(&format!(
                        "hello, {}! I was build with Twilight.",
                        msg.author.global_name.as_ref().unwrap_or(&msg.author.name)
                    ))
                    .await?;
            }
            _ => {
                println!("{event:?}");
            }
        }
    }

    Ok(())
}
