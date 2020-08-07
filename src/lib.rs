mod block;
mod epoch;
mod player;

#[cfg(test)]
mod tests {
    use crate::block::{Block, BlockId};
    use crate::epoch::EpochNumber as EpochNumberT;
    use crate::player::PlayerId;

    use rand::prelude::*;

    #[derive(Clone, Copy)]
    struct Id(u64);

    impl Id {
        fn new() -> Self {
            Id(rand::thread_rng().gen())
        }
    }

    impl BlockId for Id {}

    impl PlayerId for Id {}

    #[derive(Clone, Copy)]
    struct EpochNumber(u64);

    impl EpochNumberT for EpochNumber {
        fn genesis() -> Self {
            EpochNumber(0)
        }

        fn consecutive(&self) -> Self {
            EpochNumber(self.0 + 1)
        }
    }

    #[test]
    fn block_is_notarized_returns_true_for_genesis() {
        assert!(Block::<Id, Id, EpochNumber>::Genesis {
            id: Id::new(),
            epoch: EpochNumber::genesis(),
        }
        .is_notarized(&[]));
    }

    #[test]
    fn block_is_notarized_returns_false_for_block_with_empty_votes() {
        let players = (0..100).map(|_| Id::new()).collect::<Vec<Id>>();

        let genesis_epoch = EpochNumber::genesis();
        let genesis_block = Block::Genesis {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let child_block = Block::Child {
            id: Id::new(),
            author: Id::new(),
            parent: Box::new(genesis_block),
            votes: vec![],
            epoch: genesis_epoch.consecutive(),
        };

        assert!(!child_block.is_notarized(players.as_slice()));
    }

    #[test]
    fn block_with_two_thirds_is_notarized_returns_true() {
        let players = (0..100).map(|_| Id::new()).collect::<Vec<Id>>();

        let genesis_epoch = EpochNumber::genesis();
        let genesis_block = Block::Genesis {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let child_block = Block::Child {
            id: Id::new(),
            author: Id::new(),
            parent: Box::new(genesis_block),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        assert!(child_block.is_notarized(players.as_slice()));
    }
}
