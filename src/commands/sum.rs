use anyhow::Result;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle_sum(
    http: &HttpClient,
    args: &[&str],
    msg: &MessageCreate,
) -> Result<()> {
    let total: i64 = args
        .iter()
        .filter_map(|a| a.parse::<i64>().ok())
        .sum();

    let reply = format!("Resultado: {}", total);
    http.create_message(msg.channel_id)
        .content(&reply)
        .await?;
    Ok(())
}
