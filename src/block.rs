use crate::epoch::EpochNumber as EpochNumberT;
use crate::player::PlayerId as PlayerIdT;
use crate::vote::Vote;

pub trait BlockId {}

pub enum Block<Id: BlockId, PlayerId: PlayerIdT, EpochNumber: EpochNumberT> {
    Genesis {
        id: Id,
    },
    Child {
        id: Id,
        author: PlayerId,
        /// `None` if genesis.
        parent: Box<Block<Id, PlayerId, EpochNumber>>,
        votes: Vec<Vote<PlayerId, Id>>,
        epoch: EpochNumber,
    },
}

impl<Id: BlockId, PlayerId: PlayerIdT, EpochNumber: EpochNumberT> Block<Id, PlayerId, EpochNumber> {
    pub fn is_notarized(&self) -> bool {
        match self {
            Block::Genesis { .. } => true,
            Block::Child { .. } => false,
        }
    }

    pub fn is_notarized_chain(&self) -> bool {
        false
    }
}
