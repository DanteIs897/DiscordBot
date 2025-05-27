use anyhow::Result;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle_ping(
    http: &HttpClient,
    msg: &MessageCreate,
) -> Result<()> {
    http.create_message(msg.channel_id)
        .content("🏓 Pong!")
        .await?;
    Ok(())
}
