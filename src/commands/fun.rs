use std::time::Duration;
use rand::Rng;

use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{ContentSafeOptions, content_safe};
use serenity::framework::standard::{Args,CommandResult};


#[command]
#[description = "You get... fricked?"]
pub async fn frick(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "You got epicly fricked.").await?;

    Ok(())
}

#[command]
#[description = "Mention a user to kill them!"]
pub async fn kill(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    match _args.single_quoted::<String>() {
        Ok(x) => {
            let array: [&str; 6] = ["died", "bit the dust", "gave up on life", "cried to their death", "farted too hard", "fell off"];
            let rng = rand::thread_rng().gen_range(0..=5);

            let mention_name = &msg.mentions[0].name;
            let message_content = format!("{x} {y}", x = mention_name, y = array[rng]);

            msg.channel_id.say(&ctx.http, &message_content).await?;
            println!("{}", x);
            return Ok(());
        },
        Err(_) => {
            msg.reply(ctx, "An argument is required to run this command. Make sure a user (ping) is passed in as an argument.").await?;
            return Ok(());
        }
    }
}

#[command]
#[description = "Try your luck! You get one hour of good luck if you get bababooey."]
pub async fn bababooey(ctx: &Context, msg: &Message) -> CommandResult {
    let array: [&str; 2] = ["bababooey", "no bababooey :("];
    let baba_or_nobaba = rand::thread_rng().gen_range(0..=1);

    msg.reply(ctx, array[baba_or_nobaba]).await?;

    Ok(())
}

#[command]
#[description = "Play a guessing game."]
pub async fn guess(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let mut tries = 5;
    let _ = msg.reply(ctx, "You have 10 seconds to guess the number between 1 and 100!").await;
    let super_secret_number = rand::thread_rng().gen_range(0..=100).to_string();
    println!("{}", super_secret_number);

    while tries > 0 {
        if let Some(answer) = &msg.author.await_reply(ctx).timeout(Duration::from_secs(10)).await {
            if answer.content.to_lowercase() == super_secret_number {
                let _ = answer.reply(ctx, "That's correct!").await;
                return Ok(());
            } else {
                tries -= 1;
                let _ = answer.reply(ctx, format!("Wrong, try again! Tries: {x}", x = tries.to_string())).await;
            }
        } else {
            let _ = msg.reply(ctx, "No answer within 10 seconds.").await;
            return Ok(())
        };
    }
    msg.reply(ctx, "You ran out of tries!").await?;

    return Ok(())
}

#[command]
#[description = "Send an anonymous message 😨"]
pub async fn deletethis(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    match _args.single_quoted::<String>() {
        Ok(x) => {
            let settings = if let Some(guild_id) = msg.guild_id {
                ContentSafeOptions::default()
                    .clean_channel(false)
                    .display_as_member_from(guild_id)
            } else {
                ContentSafeOptions::default().clean_channel(false).clean_role(true)
            };

            let content = content_safe(&ctx.cache, x, &settings, &msg.mentions);

            msg.channel_id.say(&ctx.http, &content).await?;
            msg.delete(&ctx.http).await?;

            return Ok(());
        },
        Err(_) => {
            msg.reply(ctx, "An argument is required to run this command.").await?;
            return Ok(());
        },
    };
}