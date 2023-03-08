use rand::Rng;

use roux::Subreddit;
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{Args,CommandResult};

#[command]
#[description = "Get posts from r/memes or a subreddit of your choice."]
#[cfg_attr(feature = "async", tokio::main)]
pub async fn meme(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let mut subreddit_name = _args.message();
    if _args.is_empty() == true {
        subreddit_name = "memes";
    }
    let subreddit = Subreddit::new(subreddit_name);
    let index = rand::thread_rng().gen_range(0..=25);
    let post = subreddit.hot(25, None).await.unwrap();

    if subreddit.about().await.unwrap().over18 == Some(true) || post.data.children.get(index).unwrap().data.over_18 == true{
        if Channel::is_nsfw(&msg.channel(&ctx.http).await?) == false{
            msg.reply(ctx, "This is an NSFW subreddit/post, while this channel is not.").await?;
            return Ok(());
        } 
    }

    let post_upvotes = post.data.children.get(index).unwrap().data.ups.to_string();
    let post_comments = post.data.children.get(index).unwrap().data.num_comments.to_string();
    let post_title = post.data.children.get(index).unwrap().data.title.clone();
    let post_permalink = post.data.children.get(index).unwrap().data.permalink.clone();
    let wrapped_url = post.data.children.get(index).unwrap().data.url.clone();
    let unwrapped_url = wrapped_url.as_ref().unwrap();

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| e
            .title(format!("{}", post_title))
            .url(format!("{}", post_permalink))
            .description(format!("👍 ``{up}`` 💬 ``{com}``", up = post_upvotes, com = post_comments))
            .image(format!("{}", unwrapped_url))
            .footer(|f| {
                f.text( format!("Stolen from r/{}", subreddit_name) );
                f
            })
        )
    }).await?;
    Ok(())
}