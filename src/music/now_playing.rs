use poise::serenity_prelude::Color;
use std::time::Duration;

use crate::types::{CommandResult, Context};

#[poise::command(slash_command)]
pub async fn now_playing(ctx: Context<'_>) -> CommandResult {
    let lavalink_client = ctx.data().lavalink.clone();
    let player_context = lavalink_client.get_player_context(ctx.guild_id().unwrap());

    let Some(player) = player_context else {
        ctx.say("Bot isn't connected to some VC channel").await?;
        return Ok(());
    };

    let Some(current_track) = player.get_player().await.unwrap().track else {
        ctx.say("There isn't a song playing").await?;
        return Ok(());
    };

    let max_track_duration =
        formatter_minutes_second_format(Duration::from_millis(current_track.info.length).as_secs());
    let track_title = current_track
        .info
        .title
        .replace(&current_track.info.author, "");

    if let Some(uri) = current_track.info.uri.clone() {
        if uri.contains("youtube") {
            println!("{}", uri);
            ctx.send(|message| {
                message.ephemeral(true).reply(true).embed(|embed| {
                    embed
                        .color(Color::BLUE)
                        .description(format!(
                            "{}\n**Duration**: `{}`\n**Source**: [{}]({})",
                            current_track.info.title,
                            max_track_duration,
                            current_track.info.source_name,
                            uri
                        ))
                        .thumbnail(current_track.info.artwork_url.unwrap())
                        .footer(|footer| {
                            footer
                                .text("Powered by Mistik")
                                .icon_url(ctx.cache().current_user().avatar_url().unwrap())
                        })
                })
            })
            .await?;

            return Ok(());
        }
    }

    ctx.send(|message| {
        message.ephemeral(true).reply(true).embed(|embed| {
            embed
                .color(Color::BLUE)
                .description(format!(
                    "{}\n**Duration**:`{}`\n**Source**: [{}]({})",
                    track_title,
                    max_track_duration,
                    current_track.info.source_name,
                    current_track.info.uri.unwrap()
                ))
                .thumbnail(current_track.info.artwork_url.unwrap())
                .author(|author| author.name(format!("**{}**", current_track.info.author)))
                .footer(|footer| {
                    footer
                        .text("Powered by Mistik")
                        .icon_url(ctx.cache().current_user().avatar_url().unwrap())
                })
        })
    })
    .await?;

    Ok(())
}

fn formatter_minutes_second_format(track_duration: u64) -> String {
    let minutes = (track_duration / 60).to_string();
    let mut seconds = (track_duration % 60).to_string();

    if seconds == "0" {
        seconds = "00".to_string();
    }

    format!("{}:{:.2}", minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::formatter_minutes_second_format;

    #[test]
    fn it_should_return_max_duration_song() {
        let result = formatter_minutes_second_format(180);
        assert_eq!("3:00", result);
    }
}
