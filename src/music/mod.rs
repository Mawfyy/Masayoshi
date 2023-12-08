pub mod leave;
pub mod r#loop;
pub mod now_playing;
pub mod play;
pub mod resume;
pub mod skip;
pub mod stop;

use crate::types::{CommandResult, Context};

pub async fn join_bot_vc(ctx: &Context<'_>) -> CommandResult {
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
