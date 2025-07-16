use std::{env, error::Error};
use twilight_gateway::{EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("DISCORD_TOKEN")?;
    let intents = Intents::GUILD_MESSAGES;
    let mut shard = Shard::new(ShardId::ONE, token, intents);

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            println!("could not receive event");
            continue;
        };
        println!("{event:?}");
    }

    Ok(())
}
