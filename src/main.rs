use serenity::{async_trait, client::{
        Client,
        Context,
        EventHandler
    }, framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    }, framework::standard::Args, model::prelude::Activity, utils::ContentSafeOptions, model::prelude::OnlineStatus, model::prelude::Ready, model::{
        channel::{
            Message
        }
    }, utils::content_safe};

use dotenv::dotenv;
use std::env;

#[group]
#[commands(ping, download)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.tag());

        ctx.set_presence(Some(Activity::streaming("Dot Downloads", "https://id.twitch.tv/oauth2/authorize?client_id=jpoy9odw2cvu1hwmyrv5ur5q8nd0y6&force_verify=true&lang=en&login_type=login&redirect_uri=https://dothq.co&response_type=c")), OnlineStatus::DoNotDisturb).await;
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Token environment variable is not set");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong").await?;

    Ok(())
}

#[command]
async fn download(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let platform = content_safe(&ctx.cache, &args.rest(), &ContentSafeOptions::default()).await;

    if platform == "windows" {
        msg.reply(ctx, "No builds for Windows yet.").await?;
    } else if platform == "ubuntu" {
        msg.reply(ctx, "linux = sex").await?;
    } else if platform == "macos" {
        msg.reply(ctx, "what big corporation os is bad").await?;
    } else {
        msg.reply(ctx, "Specify what version you want to download. `windows, macos, ubuntu`").await?;
    }

    Ok(())
}