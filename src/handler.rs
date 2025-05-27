use anyhow::Result;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::commands::dispatch;

pub async fn handle_message(
    http: &HttpClient,
    msg: &MessageCreate,
) -> Result<()> {
    if msg.author.bot {
        return Ok(());
    }

    let content = msg.content.trim();
    if !content.starts_with('!') {
        return Ok(());
    }

    let mut parts  = content[1..].split_whitespace();
    let command    = parts.next().unwrap_or_default(); // !ping 
    let args: Vec<&str> = parts.collect();

    // println!("Mensaje: {:?}", msg);
    dispatch(http, command, &args, msg).await
}
