use lavalink_rs::prelude::{QueueMessage, SearchEngines, TrackInQueue, TrackLoadData};

use crate::{
    music::join_bot_vc,
    types::{CommandResult, Context},
};

#[poise::command(slash_command)]
pub async fn play(ctx: Context<'_>, name: String) -> CommandResult {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();

    join_bot_vc(&ctx).await?;

    let Some(player) = lava_client.get_player_context(guild_id) else {
        ctx.say("The bot needs to join first in the voice channel")
            .await?;
        return Ok(());
    };

    let query = get_query(name);

    let loaded_tracks = lava_client.load_tracks(guild_id, &query).await?;

    let total_tracks = player.get_queue().await.unwrap().len() + 1;

    let tracks: TrackInQueue = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => x.into(),
        Some(TrackLoadData::Search(x)) => x[0].clone().into(),
        _ => {
            ctx.say("There isn't results from that query").await?;
            return Ok(());
        }
    };

    player.set_queue(QueueMessage::PushToBack(tracks.clone()))?;

    let track_info = &tracks.track.info;

    if player.get_player().await.unwrap().track.is_none() {
        ctx.send(|message| {
            message
                .content(format!(
                    "Playing {} - {}",
                    track_info.author, track_info.title
                ))
                .ephemeral(true)
                .reply(false)
        })
        .await?;
    } else {
        ctx.send(|message| {
            message
                .content(format!(
                    "Added to queue {} - {} with position #{}",
                    track_info.author, track_info.title, total_tracks
                ))
                .ephemeral(true)
                .reply(false)
        })
        .await?;
    }

    if player.get_player().await.unwrap().track.is_none() {
        player.skip()?;
    }

    Ok(())
}

fn get_query(track: String) -> String {
    if track.starts_with("http") {
        track
    } else if track.starts_with("https://www.youtube") {
        SearchEngines::YouTube.to_query(&track).unwrap()
    } else {
        SearchEngines::Spotify.to_query(&track).unwrap()
    }
}
