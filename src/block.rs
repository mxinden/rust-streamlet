use crate::epoch::EpochNumber as EpochNumberT;
use crate::player::PlayerId as PlayerIdT;

pub trait BlockId {}

pub enum Block<Id: BlockId, PlayerId: PlayerIdT, EpochNumber: EpochNumberT> {
    Genesis {
        id: Id,
        epoch: EpochNumber,
    },
    Child {
        id: Id,
        author: PlayerId,
        /// `None` if genesis.
        parent: Box<Block<Id, PlayerId, EpochNumber>>,
        votes: Vec<PlayerId>,
        epoch: EpochNumber,
    },
}

impl<Id: BlockId, PlayerId: PlayerIdT, EpochNumber: EpochNumberT> Block<Id, PlayerId, EpochNumber> {
    pub fn is_notarized(&self, players: &[PlayerId]) -> bool {
        match self {
            Block::Genesis { .. } => true,
            Block::Child { votes, .. } => votes.len() >= players.len() * 2 / 3,
        }
    }

    pub fn is_notarized_chain(&self, players: Vec<PlayerId>) -> bool {
        false
    }
}
