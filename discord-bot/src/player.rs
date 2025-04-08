use rustble::traits::Player;
use serenity::all::{ChannelId, CreateMessage, MessageBuilder, UserId};
use serenity::prelude::Context;

#[derive(Debug, Clone, PartialEq)]
pub struct DiscordPlayer{
    player: Option<UserId>,
    lost: bool,
    balance: i64
}

impl DiscordPlayer{
    pub fn new(user: UserId) -> Self{
        DiscordPlayer{player: Some(user), lost: false, balance: 0}
    }

    pub async fn send_info(&self, ctx: &Context, channel: ChannelId){
        if let Some(player) = &self.player{
            let mut builder = MessageBuilder::new();
            builder.user(player);
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
}

impl Player for DiscordPlayer{
    fn lose(&mut self, bet: i64){
        self.lost = true;
        self.balance -= bet;
    }

    fn win(&mut self, bet: i64){
        self.balance += bet;
    }

    fn has_lost(self) -> bool {
        self.lost
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Casino{
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

    fn has_lost(self) -> bool {
        self.lost
    }
}