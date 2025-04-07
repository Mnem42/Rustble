use cassandra_cpp::{Cluster, Session};
use player::{Casino, DiscordPlayer};
use rustble::games::rr::RR;
use rustble::randomisers::SimpleRandom;
use rustble::traits::Player;
use serenity::all::{ApplicationId, CreateMessage, GatewayIntents, GuildId, MessageBuilder, Ready, UserId};
use serenity::{async_trait, Client};
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};

#[derive(Debug)]
enum ReaderErr {
    ReadingErr
}

mod player;
struct Handler{
    dbsession: Session
}

impl Handler{
    pub fn new(dbsession: Session) -> Self{
        Handler {
            dbsession
        }
    }
    pub async fn add_player<T: Player + Clone>(&self, p: T, identifier: &str) {
        let mut statement = self.dbsession.statement("INSERT INTO player.details (type, id) VALUES (?, ?);");
        statement.bind_string(0, identifier).unwrap();
        statement.bind_int64(1, p.get_balance()).unwrap();
        statement.execute().await.unwrap();
    }

    pub async fn get_discordplayer(&self, id: u64) -> Result<DiscordPlayer,ReaderErr>{
        let query = "SELECT id, name FROM employee.details WHERE id = ?";
        let mut statement = self.dbsession.statement(query);

        statement.bind_string(0,id.to_string().as_str()).unwrap();
        let result = statement.execute().await.unwrap();

        if result.row_count() == 0{
            self.add_player(DiscordPlayer::new(UserId::new(id),100), "dsc-player").await;
        }
        let row = result.first_row().unwrap();
    
        if row.get_column_by_name("type").unwrap().get_str().unwrap() == "dsc-player"{
            let id = row.get_column_by_name("id").unwrap().get_i64().unwrap();
            let bal = row.get_column_by_name("bal").unwrap().get_i64().unwrap();
            Ok(DiscordPlayer::new(UserId::new(id.try_into().unwrap()), bal))
        }
        else{
            Err(ReaderErr::ReadingErr)
        }
    }

    pub async fn get_casinoplayer(&self, id: u64) -> Result<Casino, ReaderErr>{
        let query = "SELECT id, name FROM employee.details WHERE id = ?";
        let mut statement = self.dbsession.statement(query);

        statement.bind_string(0,id.to_string().as_str()).unwrap();
        let result = statement.execute().await.unwrap();

        if result.row_count() == 0{
            self.add_player(Casino::new(GuildId::new(id),100),"casino").await;
        }
        let row = result.first_row().unwrap();

        if row.get_column_by_name("type").unwrap().get_str().unwrap() == "casino"{
            let id = row.get_column_by_name("id").unwrap().get_i64().unwrap();
            let bal = row.get_column_by_name("bal").unwrap().get_i64().unwrap();
            Ok(Casino::new(GuildId::new(id.try_into().unwrap()),bal))
        }
        else{
            Err(ReaderErr::ReadingErr)
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
        if msg.content.starts_with("!play-single") {
            let bet = match msg.content.split_whitespace().skip(1).next(){
                Some(x) => x.parse().unwrap_or(0),
                None => 0
            };

            let mut game: RR<player::GenericPlayer, SimpleRandom> = RR::new();
            
            let player = self.get_discordplayer(msg.author.id.get()).await.unwrap();
            game.add_player(self.get_casinoplayer(msg.guild_id.unwrap().get()).await.unwrap().to_generic_player());
            let _ = game.add_player(player.to_generic_player());
            let _ = game.play(bet);

            player.send_info(&ctx,msg.channel_id).await;
        }

        if msg.content.starts_with("!about"){
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
    let cassandra_host = "localhost"; // localhost for cargo build (OR) cassandra for docker build
    let user = std::env::var("DB_USER").expect("Expected a DB username in the environment");
    let password = std::env::var("DB_PASSWORD").expect("Expected a DB password in the environment");

    let mut cluster = Cluster::default();
    cluster.set_credentials(user.as_str(), password.as_str()).unwrap();
    cluster.set_contact_points(cassandra_host).unwrap();
    let session = cluster.connect().await.unwrap();

    session.execute("CREATE KEYSPACE IF NOT EXISTS players WITH replication = {'class':'SimpleStrategy', 'replication_factor' : 1};").await.unwrap();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = std::env::var("APPLICATION_ID")
        .expect("Expected an Application Id in the environment")
        .parse()
        .expect("Application Id must be a valid u64");

    let mut client = Client::builder(&token,GatewayIntents::all())
        .event_handler(Handler::new(session))
        .application_id(ApplicationId::new(application_id))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}