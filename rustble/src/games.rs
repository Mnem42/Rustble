pub mod rr{
    use crate::traits::{Player,Randomiser};
    use crate::{Debug,Error};

    #[derive(Debug, Clone)]
    pub struct RR<P, R>
        where P: Player, R: Randomiser
    {
        players: Vec<P>,
        randomiser: R
    }

    impl<P, R> RR<P, R>
        where P: Player + Clone, R:Randomiser
    {
        pub fn new() -> Self{
            RR {
                players: vec![],
                randomiser: R::new()
            }
        }

        pub fn get_players(&self) -> &Vec<P>{
            &self.players
        }

        pub fn add_player(&mut self, player: &P) -> &P{
            self.players.push(player.clone());
            self.players.last().unwrap()
        }

        pub fn set_players(&mut self, players: &[P]){
            self.players = players.to_vec();
        }

        pub fn play(&mut self, bet: i64) -> Result<&P,Error>{
            if self.players.len() == 1 {
                let barrel = self.randomiser.random_range(0, 20);
                if barrel%2 == 0 {
                    return Ok({
                        self.players[0].lose(bet);
                        &self.players[0]
                    });
                }
                else{
                    return Ok({
                        self.players[0].win(bet);
                        &self.players[0]
                    });
                }
            }

            while self.players.len() > 1{
                let barrel = self.randomiser.random_range(0, 6);

                if barrel == 0 {
                    self.players.pop().unwrap().lose(bet);
                }
            };
            return Ok({
                self.players[0].win(bet);
                &self.players[0]
            });
            
        }
    }
}