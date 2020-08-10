pub trait BlockId: Clone {}

#[derive(Clone)]
pub enum Block<Id, PlayerId, EpochNumber> {
    Genesis {
        id: Id,
        epoch: EpochNumber,
    },
    Child {
        id: Id,
        author: PlayerId,
        parent: Id,
        votes: Vec<PlayerId>,
        epoch: EpochNumber,
    },
}

impl<Id: BlockId, PlayerId, EpochNumber> Block<Id, PlayerId, EpochNumber> {
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
}
