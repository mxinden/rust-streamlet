use std::time::Instant;

use crate::epoch::Epoch;
use crate::player::PlayerId;

/// Describes at what time its which players tern to be the leader.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Schedule {
    start: Instant,
    num_players: u8,
}

impl Schedule {
    pub fn new(start: Instant, num_players: u8) -> Self {
        Schedule {
            start,
            num_players,
        }
    }
    pub fn leader(&self, time: Instant) -> PlayerId {
        From::from(time.saturating_duration_since(self.start).as_secs() % self.num_players as u64)
    }

    pub fn epoch(&self, time: Instant) -> Epoch {
        From::from(time.saturating_duration_since(self.start).as_secs())
    }
}
