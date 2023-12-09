use crate::types::{CommandResult, Context};

#[poise::command(slash_command)]
pub async fn remove_from_position(ctx: Context<'_>, position: usize) -> CommandResult {
    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(ctx.guild_id().unwrap()) else {
        ctx.say("The bot needs to join first in the voice channel")
            .await?;
        return Ok(());
    };

    let Ok(queue) = player.get_queue().await else {
        ctx.say("You can't remove tracks if the queue is empty.")
            .await?;
        return Ok(());
    };

    println!("{:?}", &queue);

    if let Some(song) = queue.get(position) {
        player.set_queue(lavalink_rs::player_context::QueueMessage::Remove(
            position - 1,
        ))?;
        ctx.say(format!("removed {}", song.track.info.title))
            .await?;
    } else {
        ctx.say("You can't remove invalided positions").await?;
    }

    return Ok(());
}
