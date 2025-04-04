pub mod rr{
    use std::sync::Arc;
    use crate::traits::{Player,Randomiser};
    use crate::{Debug,Error};

    #[derive(Debug)]
    pub struct RR<P, R>
        where P: Player + Clone, R: Randomiser
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

        pub fn get_players(&self) -> &Vec<&mut P>{
            &self.players
        }

        pub fn add_player(&mut self, player: &'a mut P) -> &mut Self{
            self.players.push(player);
            self
        }

        pub fn play(&mut self) -> Result<&P,Error>{
            if self.players.len() == 1 {
                let barrel = self.randomiser.random_range(0, 20);
                if barrel%6 == 0 {
                    return Ok(self.players[0].lose());
                }
                else{
                    return Ok(self.players[0]);
                }
            }

            while self.players.len() > 1{
                let barrel = self.randomiser.random_range(0, 6);

                if barrel == 0 {
                    self.players.pop().unwrap().lose();
                }
            };
            Ok(self.players[0].win())
            
        }
    }
}