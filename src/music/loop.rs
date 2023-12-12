use crate::types::{CommandResult, Context, Track};

#[poise::command(slash_command)]
pub async fn enable_loop(ctx: Context<'_>) -> CommandResult {
    println!("{:?}", &ctx.guild_id().unwrap());
    let lava_client = ctx.data().lavalink.clone();
    let player_context = lava_client
        .get_player_context(ctx.guild_id().unwrap())
        .unwrap();
    let track_data = player_context.get_player().await.unwrap().track.unwrap();
    player_context.user_data.write().insert::<Track>(Track {
        enable_loop: true,
        track_data: Some(track_data),
    });
    ctx.say("Enabled").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn disable_loop(ctx: Context<'_>) -> CommandResult {
    let lava_client = ctx.data().lavalink.clone();
    let player_context = lava_client
        .get_player_context(ctx.guild_id().unwrap())
        .unwrap();
    let track_data = player_context.get_player().await.unwrap().track.unwrap();
    player_context.user_data.write().insert::<Track>(Track {
        enable_loop: false,
        track_data: Some(track_data),
    });
    ctx.say("Disabled!").await?;

    Ok(())
}
