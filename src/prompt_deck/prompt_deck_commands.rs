use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

#[group]
#[commands(test_draw)]
pub struct PromptDeckGroup;

#[command]
pub async fn test_draw(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "whelp, this is a start").await?;
    Ok(())
}
