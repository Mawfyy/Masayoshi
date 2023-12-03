use crate::types::{CommandResult, Context};

#[poise::command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> CommandResult {
    let guild_id = ctx.guild_id().unwrap();
    let player_context = ctx.data().lavalink.clone().get_player_context(guild_id);

    let Some(player) = player_context else {
        ctx.say("The bot ins't joined!!").await?;
        return Ok(());
    };

    player.skip()?;
    ctx.say("Skipped!!").await?;

    Ok(())
}
