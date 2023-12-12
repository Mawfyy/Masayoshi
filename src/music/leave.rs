use crate::types::{CommandResult, Context};

#[poise::command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> CommandResult {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap();
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
