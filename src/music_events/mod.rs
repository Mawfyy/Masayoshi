use hook::hook;
use lavalink_rs::{
    model::events::{self, TrackEnd},
    prelude::*,
};

use crate::types::Track;

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    println!("{:?} -> {:?}", session_id, event);
}

#[hook]
pub async fn track_start(_: LavalinkClient, _: String, _: &events::TrackStart) {
    println!("Track start");
}

#[hook]
pub async fn track_end(client: LavalinkClient, _: String, track_ended: &TrackEnd) {
    println!("{:?}", &track_ended.guild_id);
    let player = client.get_player_context(track_ended.guild_id).unwrap();
    let data = player.user_data.read();
    let track = data.get::<Track>().unwrap();

    println!("{}", track.enable_loop);
    if track.enable_loop == true {
        let _ = player.set_queue(QueueMessage::PushToFront(track_ended.track.clone().into()));
    }
}
