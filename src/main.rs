mod bot;
mod handler;
mod commands;
mod utils;

use dotenvy::dotenv;
use std::env;
use utils::logger::init;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let _ = bot::start(token).await;
}
