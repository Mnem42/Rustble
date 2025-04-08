use player::{Casino, DiscordPlayer};
use rustble::games::rr::RR;
use rustble::randomisers::SimpleRandom;
use rustble::traits::*;
use serenity::all::{ApplicationId, CreateMessage, Message, MessageBuilder, Ready, User, UserId};
use serenity::async_trait;
use serenity::model::{timestamp, user};
use serenity::prelude::*;
use std::sync::{Arc, Mutex, TryLockError};

mod player;


struct Handler{
    players: Arc<Mutex<Vec<DiscordPlayer>>>
}

impl Handler{
    pub fn new() -> Self{
        Handler { players: Arc::new(Mutex::new(vec![])) }
    }

    pub fn add_player(&self, x:DiscordPlayer){
        self.players.try_lock().unwrap().push(x);
    }

    fn get_player_index(&self, id:UserId) -> Option<usize>{
        let tmp = self.players.lock().unwrap();
        tmp.iter().position(|x| x.get_id() == id.get())
    }

    pub async fn get_discord_player(&self, id: UserId) -> Option<DiscordPlayer>{
        let player = &self.players.try_lock().unwrap()[self.get_player_index(id)?];
        Some(DiscordPlayer::new(id,player.get_balance()))
    }

    pub async fn set_player_balance(&self, id: u64, bal:i64 ){
        self.get_discord_player(UserId::new(id)).await.unwrap().set_balance(bal);
    }
}


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        println!("{}", msg.content);

        if msg.content.starts_with("!play-single") {
            let bet = match msg.content.split_whitespace().skip(1).next(){
                Some(x) => x.parse().unwrap_or(0),
                None => 0
            };

            let mut game: RR<player::DiscordPlayer, SimpleRandom> = RR::new();
            game.add_player(self.get_discord_player(msg.author.id).await.unwrap());
            let winner = game.play(bet).unwrap();

            println!(" A");

            self.set_player_balance(winner.get_id(), winner.get_balance()).await;

            println!(" B");
            let _ = winner.send_info(&ctx,msg.channel_id).await;

            println!(" C");
            return;
        }
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
            return;
        }
        if msg.content == "!enroll"{
            self.add_player(DiscordPlayer::new(msg.author.id,100));
            return;
        }
        if msg.content == "!about"{
            let mut builder = MessageBuilder::new();
            builder.push_line("# Commands");
            builder.push_line("`!play-single` : Plays a single player game of russian roulette");
            builder.push_line("`!about      ` : Shows this help");
            let _ = msg.channel_id.send_message(&ctx, CreateMessage::new().content(builder.to_string())).await;
            return;
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