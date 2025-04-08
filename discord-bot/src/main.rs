use player::{Casino, DiscordPlayer};
use rustble::games::rr::RR;
use rustble::randomisers::SimpleRandom;
use rustble::traits::*;
use serenity::all::{ApplicationId, CreateMessage, Message, MessageBuilder, Ready, UserId};
use serenity::async_trait;
use serenity::model::timestamp;
use serenity::prelude::*;
use std::sync::{Arc, Mutex, TryLockError};

mod player;


struct Handler{
    players: Arc<Mutex<Vec<Box<dyn IdPlayer>>>>
}

impl Handler{
    pub fn new() -> Self{
        Handler { players: Arc::new(Mutex::new(vec![])) }
    }

    pub fn add_player(&self, x:DiscordPlayer) -> Result<(), TryLockError<std::sync::MutexGuard<'_, Vec<Box<dyn IdPlayer>>>>>{
        self.players.try_lock()?.push(Box::new(x));
        Ok(())
    }
    pub fn add_casino_player(&self, x:Casino) -> Result<(), TryLockError<std::sync::MutexGuard<'_, Vec<Box<dyn IdPlayer>>>>>{
        self.players.try_lock()?.push(Box::new(x));
        Ok(())
    }

    pub fn get_discord_player(&self, id: UserId) -> Result<Option<DiscordPlayer>,TryLockError<std::sync::MutexGuard<'_, Vec<Box<dyn IdPlayer>>>>>{
        let tmp = self.players.lock()?;
        let tmp = tmp.iter().find(|x| x.get_id() == id.get());
        if let Some(x) = tmp{
            Ok(Some(DiscordPlayer::new(UserId::new(x.get_id()),x.get_balance())))
        }
        else{
            Ok(None)
        }
    }
}


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
        else if msg.content.starts_with("!enroll"){
            self.add_player(DiscordPlayer::new(msg.author.id,100)).unwrap();
        }
        else if msg.content.starts_with("!play-single") {
            let bet = match msg.content.split_whitespace().skip(1).next(){
                Some(x) => x.parse().unwrap_or(0),
                None => 0
            };

            let mut game: RR<player::DiscordPlayer, SimpleRandom> = RR::new();
            game.add_player(self.get_discord_player(msg.author.id).unwrap().unwrap());
            let _ = game.play(bet);

            let _ = game.get_players()[0].send_info(&ctx,msg.channel_id).await;
        }
        else if msg.content.starts_with("!about"){
            let mut builder = MessageBuilder::new();
            builder.push_line("# Commands");
            builder.push_line("`!play-single` : Plays a single player game of russian roulette");
            builder.push_line("`!about      ` : Shows this help");
            let _ = msg.channel_id.send_message(&ctx, CreateMessage::new().content(builder.to_string())).await;
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
        .event_handler(Handler::new())
        .application_id(ApplicationId::new(application_id))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}