use std::env;

use serenity::framework::standard::macros::command;
use rosu_v2::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{Args,CommandResult};

#[command]
#[description = "Get a user's top score in Osu!Std"]
pub async fn score(ctx: &Context, msg: &Message, _args: Args) -> CommandResult{
    let mut username = _args.message();
    if _args.is_empty() == true {
        username = "CursedBoat";
    }
    
    let client_id: u64 = 20884;
    let client_secret = env::var("OSU_CLIENT_SECRET").expect("Didn't find an Osu! client secret in the environment.");

    let osu: Osu = match Osu::new(client_id, client_secret).await {
        Ok(client) => client,
        Err(why) => panic!(
            "Failed to create client or make initial osu!api interaction: {}",
            why
        ),
    };

    let user_object = osu.user(username).await.unwrap();
    let mut scores = osu.user_scores(username)
        .mode(GameMode::Osu)
        .best()
        .offset(0)
        .limit(1)
        .await
        .unwrap_or_else(|why| panic!("Failed to get scores: {}", why));

    let score_data = scores.get_mut(0);
    let mapname_0 = &score_data.as_ref().unwrap().mapset.as_ref().unwrap().title;
    let pp_0 = &score_data.as_ref().unwrap().pp.unwrap().to_string();

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| e
            .title(format!("{}'s top play", username))
            .url(format!("https://osu.ppy.sh/users/{}", user_object.user_id))
            .description(
                format!(
                    "``Highest rank: {:?}``, ``Followers: {:?}``, ``Play count: {:?}``", 
                    user_object.highest_rank.unwrap().rank, 
                    user_object.follower_count.unwrap(), 
                    user_object.beatmap_playcounts_count.unwrap()
                )
            )
            .thumbnail(format!("{}", user_object.avatar_url))
            .field(mapname_0, format!("{} pp", pp_0), false)
        )
    }).await?;
    Ok(())
}