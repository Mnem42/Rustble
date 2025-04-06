use rustble::traits::Player;
use serenity::all::{ChannelId, CreateMessage, MessageBuilder, UserId};
use serenity::prelude::Context;

#[derive(Debug, Clone, PartialEq)]
pub struct DiscordPlayer{
    player: Option<UserId>,
    lost: bool
}


impl DiscordPlayer{
    pub fn new(user: UserId) -> Self{
        DiscordPlayer{player: Some(user), lost: false}
    }

    pub async fn send_dminfo(&self, ctx: Context, channel: ChannelId){
        if let Some(player) = &self.player{
            let mut builder = MessageBuilder::new();
            builder.user(player);
            builder.push(match self.lost{
                true =>  " You lost",
                false => " You won"
            });
            let builder = CreateMessage::new().content(builder.to_string());
            let _ = channel.send_message(&ctx, builder).await;
        }
    }
}

impl Player for DiscordPlayer{
    fn new() -> Self{
        DiscordPlayer { player: None, lost:false }
    }

    fn lose(&mut self) -> &mut Self {
        self.lost = true;
        self
    }

    fn win(&mut self) -> &mut Self {
        self
    }

    fn has_lost(self) -> bool {
        self.lost
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Casino{
    lost: bool
}

impl Player for Casino{
    fn new() -> Self{
        Casino { lost:false }
    }

    fn lose(&mut self) -> &mut Self {
        self.lost = true;
        self
    }

    fn win(&mut self) -> &mut Self {
        self
    }

    fn has_lost(self) -> bool {
        self.lost
    }
}