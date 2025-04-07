use rustble::traits::Player;
use serenity::all::{ChannelId, CreateMessage, Guild, GuildId, MessageBuilder, UserId};
use serenity::prelude::Context;

#[derive(Debug, Clone, PartialEq)]
pub struct DiscordPlayer{
    player: UserId,
    lost: bool,
    balance: i64
}

impl DiscordPlayer{
    pub fn new(user: UserId, bal: i64) -> Self{
        DiscordPlayer{player: user, lost: false, balance: bal}
    }

    pub fn to_generic_player(&self) -> GenericPlayer {
        GenericPlayer::new(self.player.get(), self.balance)
    }

    pub fn from_generic_player(x: GenericPlayer) -> Self{
        DiscordPlayer { player: x.id.into(), lost: x.clone().has_lost(), balance: x.get_balance() }
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
    fn get_balance(&self) -> i64 {
        return self.balance
    }

    fn lose(&mut self, bet: i64) {
        self.lost = true;
        self.balance -= bet;
    }

    fn win(&mut self, bet: i64) {
        self.balance += bet;
    }

    fn has_lost(self) -> bool {
        self.lost
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Casino{
    id: GuildId,
    lost: bool,
    balance: i64
}

impl Casino{
    pub fn new(id: GuildId, balance: i64) -> Self{
        Casino { id, lost: false, balance }
    }

    pub fn to_generic_player(&self) -> GenericPlayer {
        GenericPlayer::new(self.id.get(), self.balance)
    }
}

impl Player for Casino{
    fn get_balance(&self) -> i64 {
        return self.balance
    }

    fn lose(&mut self, bet: i64) {
        self.lost = true;
        self.balance -= bet;
    }

    fn win(&mut self, bet: i64) {
        self.balance += bet;
    }

    fn has_lost(self) -> bool {
        self.lost
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericPlayer{
    pub id: u64,
    lost: bool,
    balance: i64,
    pub is_player: bool
}

impl GenericPlayer{
    pub fn new(id: u64, balance: i64) -> Self{
        GenericPlayer{
            id,
            lost:false,
            balance,
            is_player: true
        }
    }

}

impl Player for GenericPlayer{
    fn get_balance(&self) -> i64 {
        return self.balance
    }

    fn lose(&mut self, bet: i64) {
        self.lost = true;
        self.balance -= bet;
    }

    fn win(&mut self, bet: i64) {
        self.balance += bet;
    }

    fn has_lost(self) -> bool {
        self.lost
    }
}