use twilight_http::request::AuditLogReason;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle_ban(
    http: &HttpClient,
    args: &[&str],
    msg: &MessageCreate,
) {
    let reason = args[1..].join(" ");
    http.create_ban(msg.guild_id.unwrap(), msg.mentions[0].id).reason(&reason).await.unwrap();
}