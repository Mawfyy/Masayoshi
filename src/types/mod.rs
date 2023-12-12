use std::sync::Arc;

use lavalink_rs::{model::track::TrackData, prelude::LavalinkClient, typemap_rev::TypeMapKey};
use tokio::sync::Mutex;

pub struct Data {
    pub lavalink: LavalinkClient,
    pub is_karaoke_enable: Arc<Mutex<bool>>,
}

pub struct Track {
    pub enable_loop: bool,
    pub track_data: Option<TrackData>,
}

impl TypeMapKey for Track {
    type Value = Track;
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type CommandResult = Result<(), Error>;
