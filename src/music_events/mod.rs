use crate::types::Context;
use hook::hook;
use lavalink_rs::{model::events, prelude::*};

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    println!("{:?} -> {:?}", session_id, event);
}

#[hook]
pub async fn track_start(client: LavalinkClient, session_id: String, event: &events::TrackStart) {
    println!("Track start");
}
