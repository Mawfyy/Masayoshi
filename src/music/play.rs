use lavalink_rs::prelude::{QueueMessage, SearchEngines, TrackInQueue, TrackLoadData};

use crate::{
    music::join_bot_vc,
    types::{CommandResult, Context},
};

#[poise::command(slash_command)]
pub async fn play(ctx: Context<'_>, name: String) -> CommandResult<()> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;
    let lava_client = ctx.data().lavalink.clone();
    join_bot_vc(&ctx).await?;

    let player_context = lava_client.get_player_context(guild_id);

    let Some(player) = player_context else {
        ctx.say("The bot needs to join first in the voice channel")
            .await?;
        return Ok(());
    };

    let query = if name.starts_with("http") {
        name
    } else if name.starts_with("https://open.spotify") {
        SearchEngines::Spotify.to_query(&name)?
    } else {
        SearchEngines::YouTube.to_query(&name)?
    };

    let loaded_tracks = lava_client.load_tracks(guild_id, &query).await?;

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

    println!("{}", player.get_queue().await.unwrap().len());

    if let Some(info) = playlist_info {
        ctx.say(format!(
            "Added playlist {}  with {} tracks",
            info.name,
            tracks.len()
        ))
        .await?;
    } else {
        let track = &tracks[0].track;
        ctx.say(format!(
            "Added to queue: {} - {} with position #{}",
            track.info.author,
            track.info.title,
            player.get_queue().await.unwrap().len() + 1
        ))
        .await?;
    }
    Ok(())
}
