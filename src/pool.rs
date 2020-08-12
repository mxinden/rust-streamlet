use crate::block::{Block, BlockId as BlockIdT};
use crate::epoch::Epoch;
use crate::player::PlayerId;

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

/// Represents a pool of blocks.
#[derive(Clone, Debug)]
pub struct Pool<BlockId> {
    blocks: HashMap<BlockId, Block<BlockId>>,
}

impl<BlockId: BlockIdT> Pool<BlockId> {
    pub fn new() -> Self {
        Pool {
            blocks: HashMap::new(),
        }
    }

    pub fn insert(&mut self, block: Block<BlockId>) {
        self.blocks.insert(block.get_id().clone(), block);
    }

    pub fn get(&self, block_id: &BlockId) -> Option<&Block<BlockId>> {
        self.blocks.get(block_id)
    }

    pub fn get_epoch(&self, block_id: &BlockId) -> Option<Epoch> {
        self.get(block_id).map(Block::get_epoch)
    }

    pub fn is_notarized_chain(&self, block_id: &BlockId, players: &[PlayerId]) -> bool {
        let block = match self.blocks.get(block_id) {
            None => return false,
            Some(block) => block,
        };

        if !block.is_notarized(players) {
            return false;
        }

        match block {
            Block::Genesis { .. } => true,
            Block::Child { parent, .. } => self.is_notarized_chain(parent, players),
        }
    }

    pub fn chain_contains(&self, head: &BlockId, block: &BlockId) -> bool {
        if head == block {
            return true;
        }

        match self.get(head) {
            Some(Block::Genesis { id, .. }) => id == block,
            Some(Block::Child { parent, .. }) => {
                if parent == block {
                    true
                } else {
                    self.chain_contains(parent, block)
                }
            }
            None => false,
        }
    }

    /// Returns the [`BlockId`] of the youngest finalized block in the chain
    /// where the head equals the provided [`block_id`].
    ///
    /// Note: This function does not explore blocks younger than the block with
    /// the provided [`BlockId`]. Thus the block with the provided [`BlockId`]
    /// might still be finalized looking at it from the perspective of a younger
    /// block.
    //
    // TODO: Maybe just `is_finalized` is more versatile?
    pub fn get_finalized<'a>(
        &'a self,
        block_id: &'a BlockId,
        players: &[PlayerId],
    ) -> Option<&'a BlockId> {
        let block = self.blocks.get(block_id)?;

        let is_notarized = block.is_notarized(players);

        match block {
            Block::Genesis { id, .. } => return Some(id),
            Block::Child { parent, .. } if !is_notarized => {
                return self.get_finalized(parent, players)
            }
            Block::Child { parent, epoch, .. } => {
                if !self.is_notarized_chain(block_id, players) {
                    return self.get_finalized(parent, players);
                }

                let (parent_parent, parent_epoch) = match self.blocks.get(parent)? {
                    Block::Genesis { .. } => return Some(parent),
                    Block::Child { parent, epoch, .. } => (parent, epoch),
                };

                let parent_parent_epoch = match self.blocks.get(parent_parent)? {
                    Block::Genesis { .. } => return Some(parent_parent),
                    Block::Child { epoch, .. } => epoch,
                };

                if parent_parent_epoch.consecutive() == *parent_epoch
                    && parent_epoch.consecutive() == *epoch
                {
                    return Some(parent);
                }

                return self.get_finalized(parent, players);
            }
        }
    }

    pub fn iter_blocks(&self) -> impl Iterator<Item = &BlockId> {
        self.blocks.keys()
    }
}

impl<BlockId> FromIterator<Block<BlockId>> for Pool<BlockId>
where
    BlockId: Eq + Hash + BlockIdT,
{
    fn from_iter<I: IntoIterator<Item = Block<BlockId>>>(iter: I) -> Self {
        Pool {
            blocks: iter.into_iter().map(|b| (b.get_id().clone(), b)).collect(),
        }
    }
}
