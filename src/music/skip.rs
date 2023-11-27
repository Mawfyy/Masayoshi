use crate::types::{CommandResult, Context};

#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> CommandResult<()> {
    let guild_id = ctx.guild_id().unwrap();
    let player_context = ctx.data().lavalink.clone().get_player_context(guild_id);
    let queue = player_context.clone().unwrap().get_queue().await.unwrap();

    let Some(player) = player_context else {
        ctx.say("The bot ins't joined!!").await?;
        return Ok(());
    };

    if queue.is_empty() {
        ctx.say("You can't skip if there isn't songs yet").await?;
    } else {
        player.skip()?;
        ctx.say("Skipped!!").await?;
    }

    Ok(())
}
