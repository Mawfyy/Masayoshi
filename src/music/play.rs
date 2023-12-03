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

    let player_context = lava_client.get_player_context(guild_id);
    let Some(player) = player_context else {
        ctx.say("The bot needs to join first in the voice channel")
            .await?;
        return Ok(());
    };

    let query = get_query(name);

    let loaded_tracks = lava_client.load_tracks(guild_id, &query).await?;

    let total_tracks = player.get_queue().await.unwrap().len() + 1;

    let mut playlist_info = None;

    let tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.into()).collect()
        }
        _ => return Ok(()),
    };

    player.set_queue(QueueMessage::Append(tracks.clone().into()))?;

    if let Some(info) = playlist_info {
        ctx.send(|message| {
            message
                .content(format!(
                    "Added playlist! with name {} and {} tracks",
                    info.name,
                    &tracks.len()
                ))
                .ephemeral(true)
                .reply(false)
        })
        .await?;
    } else {
        let track = &tracks[0].track;
        ctx.send(|message| {
            message
                .content(format!(
                    "Added to queue: {} - {} with position #{}",
                    track.info.author, track.info.title, total_tracks
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
    } else if track.starts_with("https://open.spotify") {
        SearchEngines::Spotify.to_query(&track).unwrap()
    } else {
        SearchEngines::YouTube.to_query(&track).unwrap()
    }
}
