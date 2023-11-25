use std::sync::Arc;

use lavalink_rs::prelude::LavalinkClient;
use tokio::sync::Mutex;

pub struct Data {
    pub lavalink: LavalinkClient,
    pub is_karaoke_enable: Arc<Mutex<bool>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
