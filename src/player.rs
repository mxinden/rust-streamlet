use crate::block::{Block, BlockId as BlockIdT};
use crate::pool::Pool;
use crate::schedule::Schedule;

use std::time::Instant;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct PlayerId(u64);

impl From<u64> for PlayerId {
    fn from(id: u64) -> Self {
        PlayerId(id)
    }
}

pub struct Player<BlockId> {
    id: PlayerId,
    pool: Pool<BlockId>,
    schedule: Schedule,
}

/// Messages send to a player.
pub enum Incoming {}

/// Messages send by a player.
pub enum Outgoing {}

impl<BlockId: BlockIdT> Player<BlockId> {
    pub fn new(id: PlayerId, schedule: Schedule) -> Self {
        Player {
            id,
            pool: Pool::new(),
            schedule,
        }
    }
    pub fn next(&mut self, now: Instant, incoming_msgs: Vec<Incoming>) -> Vec<Outgoing> {
        if self.schedule.leader(now) == self.id {
            println!(
                "I {:?} am the leader in epoch {:?}",
                self.id,
                self.schedule.epoch(now)
            );
        }

        vec![]
    }
}
