use crate::block::BlockId as BlockIdT;
use crate::player::Player;
use crate::schedule::Schedule;

use rand::prelude::*;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Id(u64);

impl Id {
    fn new() -> Self {
        Id(rand::thread_rng().gen())
    }
}

impl BlockIdT for Id {}

// TODO: Simulator could also just implement `Iterator` instead of `run`.
pub struct Simulator {
    players: Vec<Player<Id>>,
}

impl Simulator {
    pub fn run(&mut self) {
        loop {
            let now = Instant::now();

            for player in self.players.iter_mut() {
                player.next(now, vec![]);
            }

            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

pub struct Builder {
    num_players: u8,
}

impl Default for Builder {
    fn default() -> Self {
        Builder { num_players: 100 }
    }
}

impl Builder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Simulator {
        let schedule = Schedule::new(Instant::now(), self.num_players);
        let players = (0..self.num_players)
            .map(|id| (id as u64).into())
            .map(|id| Player::new(id, schedule))
            .collect();

        Simulator { players }
    }
}
