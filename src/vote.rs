use crate::block::BlockId as BlockIdT;
use crate::player::PlayerId as PlayerIdT;

pub struct Vote<PlayerId: PlayerIdT, BlockId: BlockIdT> {
    author: PlayerId,
    block: BlockId,
}
