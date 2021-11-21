use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

#[group]
#[commands(test_random)]
pub struct RandomizerGroup;

#[command]
pub async fn test_random(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "whelp, this is a start").await?;
    Ok(())
}
