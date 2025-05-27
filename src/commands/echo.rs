use anyhow::Result;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle_echo(
    http: &HttpClient,
    args: &[&str],
    msg: &MessageCreate,
) -> Result<()> {

    let reply = args.join(" ");

    http.create_message(msg.channel_id)
        .content(&reply)
        .await?;

    Ok(())
}
