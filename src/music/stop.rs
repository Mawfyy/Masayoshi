use crate::types::{CommandResult, Context};

#[poise::command(slash_command)]
pub async fn stop(ctx: Context<'_>) -> CommandResult {
    let guild_id = ctx.guild_id().unwrap();
    let lava_client = ctx.data().lavalink.clone();
    let player_context = lava_client.get_player_context(guild_id);

    if player_context.is_none() {
        ctx.say("There isn't song to skip").await?;
        return Ok(());
    }

    player_context.unwrap().set_pause(true).await?;
    ctx.say("Stopped!!").await?;
    Ok(())
}
