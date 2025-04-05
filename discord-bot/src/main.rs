use player::DiscordPlayer;
use rustble::games::rr::RR;
use rustble::randomisers::SimpleRandom;
use serenity::all::{ApplicationId, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

mod player;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
        if msg.content == "!play-single" {
            let mut game: RR<player::DiscordPlayer, SimpleRandom> = RR::new();
            
            let player = DiscordPlayer::new(msg.author.id);
            game.add_player(player);
            let _ = game.play();

            let _ = game.get_players()[0].send_dminfo(ctx,msg.channel_id).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = std::env::var("APPLICATION_ID")
        .expect("Expected an Application Id in the environment")
        .parse()
        .expect("Application Id must be a valid u64");

    let mut client = Client::builder(&token,GatewayIntents::all())
        .event_handler(Handler)
        .application_id(ApplicationId::new(application_id))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}