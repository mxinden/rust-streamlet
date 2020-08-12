use crate::epoch::Epoch;
use crate::player::PlayerId;

use std::hash::Hash;

pub trait BlockId: Clone + Eq + Hash {}

#[derive(Clone, Debug)]
pub enum Block<Id> {
    Genesis {
        id: Id,
        epoch: Epoch,
    },
    Child {
        id: Id,
        author: PlayerId,
        parent: Id,
        votes: Vec<PlayerId>,
        epoch: Epoch,
    },
}

impl<Id: BlockId> Block<Id> {
    pub fn is_notarized(&self, players: &[PlayerId]) -> bool {
        match self {
            Block::Genesis { .. } => true,
            Block::Child { votes, .. } => votes.len() >= players.len() * 2 / 3,
        }
    }

    pub fn is_genesis(&self) -> bool {
        match self {
            Block::Genesis { .. } => true,
            Block::Child { .. } => false,
        }
    }

    pub fn get_id(&self) -> &Id {
        match self {
            Block::Genesis { id, .. } => id,
            Block::Child { id, .. } => id,
        }
    }

    pub fn get_epoch(&self) -> Epoch {
        match self {
            Block::Genesis { epoch, .. } => *epoch,
            Block::Child { epoch, .. } => *epoch,
        }
    }
}
