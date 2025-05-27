pub mod ping;
pub mod sum;
pub mod echo;
pub mod ban;


use anyhow::Result;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn dispatch(
    http: &HttpClient,
    command: &str,
    args: &[&str],
    msg: &MessageCreate,
) -> Result<()> {
    match command {
        "ping" => ping::handle_ping(http, msg).await,
        "sum"  => sum::handle_sum(http, args, msg).await,
        "echo" => echo::handle_echo(http, args, msg).await,
        "ban" => Ok(ban::handle_ban(http, args, msg).await),
        _ => Ok(()),
    }
}
