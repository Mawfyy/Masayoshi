mod music;
mod music_events;
mod types;

use music::{
    leave::leave,
    now_playing::now_playing,
    play::play,
    r#loop::{disable_loop, enable_loop},
    resume::resume,
    skip::skip,
    stop::stop,
};
use std::sync::Arc;

use crate::types::Data;
use dotenvy::var;
use lavalink_rs::{model::events, prelude::*};
use music_events::{ready_event, track_end, track_start};
use poise::serenity_prelude::GatewayIntents;
use poise::{Framework, FrameworkOptions};
use songbird::SerenityInit;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let lavalink_password = var("LAVALINK_PASSWORD").expect("missing LAVALINK_PASSWORD value");
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                play(),
                leave(),
                skip(),
                resume(),
                stop(),
                now_playing(),
                enable_loop(),
                disable_loop(),
            ],
            ..Default::default()
        })
        .token(var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN value"))
        .intents(GatewayIntents::all())
        .client_settings(|c| c.register_songbird())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let events = events::Events {
                    ready: Some(ready_event),
                    track_start: Some(track_start),
                    track_end: Some(track_end),
                    ..Default::default()
                };

                ctx.online().await;

                let node_local = NodeBuilder {
                    hostname: "localhost:2333".to_string(),
                    is_ssl: false,
                    events: events::Events::default(),
                    password: lavalink_password.to_string(),
                    user_id: ctx.cache.current_user_id().into(),
                    session_id: None,
                };

                let client = LavalinkClient::new(events, vec![node_local]);
                client.start().await;

                Ok(Data {
                    lavalink: client,
                    is_karaoke_enable: Arc::new(Mutex::new(false)),
                })
            })
        });

    framework.run().await.unwrap();
}
