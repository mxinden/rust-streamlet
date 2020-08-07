use crate::epoch::EpochNumber as EpochNumberT;
use crate::block::{Block, BlockId as BlockIdT};

pub trait PlayerId: Clone {}

pub struct Player<Id: PlayerId, BlockId: BlockIdT, EpochNumber: EpochNumberT> {
    id: Id,
    chains: Vec<Block<BlockId, Id, EpochNumber>>,
}
