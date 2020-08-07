use crate::epoch::EpochNumber as EpochNumberT;
use crate::player::PlayerId as PlayerIdT;

pub trait BlockId: Clone {}

#[derive(Clone)]
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
    fn parent(&self) -> Option<Self> {
        match self {
            Block::Genesis { .. } => None,
            Block::Child { parent, .. } => Some(*parent.clone()),
        }
    }
    pub fn is_notarized(&self, players: &[PlayerId]) -> bool {
        match self {
            Block::Genesis { .. } => true,
            Block::Child { votes, .. } => votes.len() >= players.len() * 2 / 3,
        }
    }

    pub fn is_notarized_chain(&self, players: &[PlayerId]) -> bool {
        !self.iter_chain().any(|b| !b.is_notarized(players))
    }

    fn iter_chain(&self) -> ChainIter<Id, PlayerId, EpochNumber> {
        ChainIter {block: Some((*self).clone())}
    }
}

struct ChainIter<Id: BlockId, PlayerId: PlayerIdT, EpochNumber: EpochNumberT> {
    block: Option<Block<Id, PlayerId, EpochNumber>>,
}

impl<Id: BlockId, PlayerId: PlayerIdT, EpochNumber: EpochNumberT> Iterator for ChainIter<Id, PlayerId, EpochNumber> {
    type Item = Block<Id, PlayerId, EpochNumber>;

    fn next(&mut self) -> Option<Self::Item> {
        let parent = self.block.as_ref().and_then(|b| b.parent());
        std::mem::replace(&mut self.block, parent)
    }
}
