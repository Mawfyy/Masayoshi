use dotenvy::var;
use lavalink_rs::prelude::{QueueMessage, SearchEngines, TrackInQueue, TrackLoadData};

use crate::types::{Context, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let data_lock = ctx.data().is_karaoke_enable.clone();
    let response = format!("{}", data_lock.lock().await);
    ctx.say(response).await?;
    Ok(())
}

/*
#[poise::command(slash_command)]
pub async fn enable_karaoke(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data().is_karaoke_enable.clone();
    let mut karaoke = data.lock().await;
    *karaoke = true;
    ctx.say("karaoke enabled").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn disable_karaoke(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data().is_karaoke_enable.clone();
    let mut karaoke = data.lock().await;
    *karaoke = false;
    ctx.say("karaoke disabled").await?;
    Ok(())
}
*/

/*
#[poise::command(slash_command)]
pub async fn lyrics(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    let lava_client = ctx.data().lavalink.clone();
    let guild_id = guild.id;
    let token = var("MUSIXMATCH_TOKEN").expect("You need to set musix token");
    let music_client = musixmatch::MusixAbgleich::new(token.as_str(), &|_error| {
        println!("Error");
    });

    let player_context = lava_client.get_player_context(guild_id);

    match player_context {
        Some(context) => {
            let current_song = context.get_player().await.unwrap();
            let title = current_song.track.unwrap().info.title.clone();
            let track = music_client
                .search_track(TrackSearchQuery::new().song_title(Some(title.as_str())))
                .await
                .unwrap();
            let lyrics = track.name();
            println!("{:?}", lyrics);
            return Ok(());
        }

        None => {
            ctx.say("There isn't song playing..").await;
            return Ok(());
        }
    }
}
*/

pub async fn join_bot(ctx: &Context<'_>) -> Result<(), Error> {
    let lava_client = ctx.data().lavalink.clone();
    let guild = ctx.guild().unwrap();
    let player_context = lava_client.get_player_context(guild.id);
    let manager = songbird::get(&ctx.serenity_context()).await.unwrap();

    if player_context.is_none() {
        let channel_id = guild.voice_states.get(&ctx.author().id);

        let Some(user_voice_state) = channel_id else {
            ctx.say("You need to connect some voice channel first")
                .await?;
            return Ok(());
        };

        let (_, connection_handler) = manager
            .join_gateway(guild.id, user_voice_state.channel_id.unwrap())
            .await;

        match connection_handler {
            Ok(connection) => {
                let _ = lava_client
                    .create_player_context(guild.id, connection)
                    .await;
                ctx.say("Joined!").await?;
                return Ok(());
            }

            Err(_) => {
                ctx.say("Error joining in the channel").await?;
                return Ok(());
            }
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn play(ctx: Context<'_>, name: String) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;
    let lava_client = ctx.data().lavalink.clone();
    join_bot(&ctx).await?;

    let player_context = lava_client.get_player_context(guild_id);

    let Some(player) = player_context else {
        ctx.say("The bot needs to join first in the voice channel")
            .await;
        return Ok(());
    };

    let query = if name.starts_with("http") {
        name
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
            player.get_queue().await.unwrap().len()
        ))
        .await?;
    }
    Ok(())
}

#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(&ctx.serenity_context()).await.unwrap();
    let handler = manager.get(guild_id);
    let lava_client = ctx.data().lavalink.clone();

    match handler {
        Some(voice_handler) => {
            let mut voice_handler_lock = voice_handler.lock().await;
            voice_handler_lock.leave().await?;
            lava_client.delete_player(guild_id).await?;
            ctx.say("Disconnected!").await?;
        }

        None => {
            ctx.say("The bot isn't connected to some channel").await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let player_context = lava_client.get_player_context(guild_id);

    if player_context.is_none() {
        ctx.say("There isn't song to skip").await;
        return Ok(());
    }

    player_context.unwrap().skip();
    ctx.say("Skipped!!").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let player_context = lava_client.get_player_context(guild_id);

    if player_context.is_none() {
        ctx.say("There isn't song to skip").await;
        return Ok(());
    }

    player_context.unwrap().set_pause(true).await?;
    ctx.say("Stopped!!").await;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let player_context = lava_client.get_player_context(guild_id);

    if player_context.is_none() {
        ctx.say("There isn't song to skip").await;
        return Ok(());
    }

    player_context.unwrap().set_pause(false).await?;
    ctx.say("Now playing!!").await;
    Ok(())
}
