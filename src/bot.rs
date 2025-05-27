use anyhow::Result;
use tracing::warn;
use twilight_http::Client as HttpClient;
use twilight_gateway::{Shard, ShardId, Intents, StreamExt, Event, EventTypeFlags};

use crate::handler::handle_message;

pub async fn start(token: String) -> Result<()> {
    let http = HttpClient::new(token.clone());

    let intents = Intents::GUILD_MESSAGES
        | Intents::DIRECT_MESSAGES
        | Intents::MESSAGE_CONTENT;

    let mut shard = Shard::new(ShardId::ONE, token, intents);

    while let Some(event_result) = shard
        .next_event(EventTypeFlags::MESSAGE_CREATE)
        .await
    {
        match event_result {
            Ok(Event::MessageCreate(msg)) => {
                if let Err(e) = handle_message(&http, &msg).await {
                    warn!("Error handling message: {}", e);
                }
            }
            Ok(_) => {}
            Err(e) => warn!("Shard error: {}", e),
        }
    }

    Ok(())
}
