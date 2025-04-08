use rustble::traits::{IdPlayer, Player};
use serenity::all::{ChannelId, CreateMessage, GuildId, MessageBuilder, UserId};
use serenity::prelude::Context;

#[derive(Debug, Clone, PartialEq)]
pub struct DiscordPlayer{
    player: UserId,
    lost: bool,
    balance: i64
}

impl DiscordPlayer{
    pub fn new(user: UserId, balance: i64) -> Self{
        DiscordPlayer{player: user, lost: false, balance}
    }

    pub async fn send_info(&self, ctx: &Context, channel: ChannelId){
        let mut builder = MessageBuilder::new();
        builder.user(self.player);
        builder.push_line(match self.lost{
            true =>  " You lost",
            false => " You won"
        });
        builder.push("Your current balance is: ");
        builder.push_bold(format!("{}",self.balance));
        let builder = CreateMessage::new().content(builder.to_string());
        let _ = channel.send_message(&ctx, builder).await;
    }
}

impl Player for DiscordPlayer{
    fn lose(&mut self, bet: i64){
        self.lost = true;
        self.balance -= bet;
    }

    fn win(&mut self, bet: i64){
        self.balance += bet;
    }

    fn has_lost(&self) -> bool {
        self.lost
    }

    fn get_balance(&self) -> i64 {
        self.balance
    }
}

impl IdPlayer for DiscordPlayer{
    fn get_id(&self) -> u64 {
        self.player.get()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Casino{
    id: GuildId,
    lost: bool,
    balance: i64
}

impl Player for Casino{
    fn lose(&mut self, bet: i64){
        self.lost = true;
        self.balance -= bet;
    }

    fn win(&mut self, bet: i64){
        self.balance += bet;
    }

    fn has_lost(&self) -> bool {
        self.lost
    }

    fn get_balance(&self) -> i64 {
        self.balance
    }
}

impl IdPlayer for Casino{
    fn get_id(&self) -> u64 {
        self.id.get()
    }
}