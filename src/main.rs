mod commands;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::{group, help};
use serenity::framework::StandardFramework;
use serenity::framework::standard::{
    Args,
    CommandGroup,
    CommandResult,
    HelpOptions,
    help_commands
};

use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::prelude::{
    Activity,
    Message,
    UserId
};

use tracing::{error, info};
use tokio::sync::Mutex;

use crate::commands::fun::*;
use crate::commands::reddit::*;
use crate::commands::osu::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        ctx.set_activity(Activity::watching("anime")).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[prefixes("fun")]
#[description = "A group of commands to have fun with!"]
#[summary = "Have fun"]
#[default_command(guess)]
#[commands(frick, deletethis, kill, guess, bababooey)]
struct Fun;

#[group]
#[prefixes("reddit")]
#[description = "A group of commands to mess around with reddit!"]
#[summary = "Reddit commands"]
#[default_command(meme)]
#[commands(meme)]
struct Reddit;

#[group]
#[prefixes("osu")]
#[description = "A group of commands to interact with Osu!"]
#[summary = "Osu! commands."]
#[default_command(score)]
#[commands(score)]
struct Osu;

#[help]
#[individual_command_tip = "If you want more information about a specific command, just pass the command as argument.\n\
For example: ~help fun guess\n"]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("~"))
            .help(&MY_HELP)
            .group(&FUN_GROUP)
            .group(&REDDIT_GROUP)
            .group(&OSU_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}