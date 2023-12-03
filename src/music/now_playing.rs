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

    let current_minute_track = formatter_minutes_second_format(
        Duration::from_millis(current_track.info.position).as_secs(),
    );

    let max_track_duration =
        formatter_minutes_second_format(Duration::from_millis(current_track.info.length).as_secs());

    ctx.send(|message| {
        message.ephemeral(true).reply(true).embed(|embed| {
            embed
                .color(Color::BLUE)
                .description(format!(
                    "{} | [{}/{}]",
                    current_track.info.author, current_minute_track, max_track_duration
                ))
                .thumbnail(current_track.info.artwork_url.unwrap())
                .author(|author| author.name(current_track.info.author))
        })
    })
    .await?;

    Ok(())
}

fn formatter_minutes_second_format(track_duration: u64) -> String {
    let minutes = track_duration / 60;
    let seconds = track_duration % 60;
    format!("{}:{:.2}", minutes, seconds)
}

#[cfg(test)]
mod tests {
    use crate::music::now_playing::formatter_minutes_second_format;

    #[test]
    fn it_should_return_minutes_format() {
        let minutes = formatter_minutes_second_format(220);
        assert_eq!("3:40", minutes);
    }
}
