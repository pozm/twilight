use std::env;
use twilight_gateway::{Event, Intents, Shard, ShardId, Command};
use twilight_model::{gateway::payload::outgoing::RequestGuildMembers, id::Id};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let mut shard = Shard::new(
        ShardId::ONE,
        env::var("DISCORD_TOKEN")?,
        Intents::GUILD_MEMBERS | Intents::GUILDS,
    );

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        match event {
            Event::Ready(rdy) => {
                println!("Connected as {}", rdy.user.name);
            }
            Event::MessageCreate(msg) => {
                println!("{}: {}", msg.author.name, msg.content);
            }
            _ => {}
        }
    }

    Ok(())
}
