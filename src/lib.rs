pub mod block;
pub mod epoch;
pub mod player;
pub mod pool;

#[cfg(test)]
mod tests {
    use crate::block::{Block, BlockId};
    use crate::epoch::EpochNumber as EpochNumberT;
    use crate::player::PlayerId;

    use rand::prelude::*;
    use std::iter::FromIterator;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    struct Id(u64);

    impl Id {
        fn new() -> Self {
            Id(rand::thread_rng().gen())
        }
    }

    impl BlockId for Id {}

    impl PlayerId for Id {}

    #[derive(Clone, Copy, PartialEq)]
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
        let genesis_block = Block::Genesis::<Id, Id, EpochNumber> {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let child_block = Block::Child {
            id: Id::new(),
            author: Id::new(),
            parent: genesis_block.get_id().clone(),
            votes: vec![],
            epoch: genesis_epoch.consecutive(),
        };

        assert!(!child_block.is_notarized(players.as_slice()));
    }

    #[test]
    fn block_with_two_thirds_is_notarized_returns_true() {
        let players = (0..100).map(|_| Id::new()).collect::<Vec<Id>>();

        let genesis_epoch = EpochNumber::genesis();
        let genesis_block = Block::Genesis::<Id, Id, EpochNumber> {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let child_block = Block::Child {
            id: Id::new(),
            author: Id::new(),
            parent: genesis_block.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        assert!(child_block.is_notarized(players.as_slice()));
    }

    #[test]
    fn is_notarized_chain_all_blocks_notarized() {
        let players = (0..100).map(|_| Id::new()).collect::<Vec<Id>>();

        let genesis_epoch = EpochNumber::genesis();
        let genesis_block = Block::Genesis::<Id, Id, EpochNumber> {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let first_child = Block::Child {
            id: Id::new(),
            author: Id::new(),
            parent: genesis_block.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        let second_child_id = Id::new();
        let second_child = Block::Child {
            id: second_child_id.clone(),
            author: Id::new(),
            parent: first_child.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        let p = crate::pool::Pool::from_iter(vec![genesis_block, first_child, second_child]);

        assert!(p.is_notarized_chain(&second_child_id, players.as_slice()));
    }

    #[test]
    fn is_notarized_chain_one_block_not_notarized() {
        let players = (0..100).map(|_| Id::new()).collect::<Vec<Id>>();

        let genesis_epoch = EpochNumber::genesis();
        let genesis_block = Block::Genesis::<Id, Id, EpochNumber> {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let first_child = Block::Child {
            id: Id::new(),
            author: Id::new(),
            parent: genesis_block.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        // Not notarized.
        let second_child = Block::Child {
            id: Id::new(),
            author: Id::new(),
            parent: first_child.get_id().clone(),
            votes: players.iter().take(60).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        let third_child_id = Id::new();
        let third_child = Block::Child {
            id: third_child_id.clone(),
            author: Id::new(),
            parent: second_child.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        let p = crate::pool::Pool::from_iter(vec![
            genesis_block,
            first_child,
            second_child,
            third_child,
        ]);

        assert!(!p.is_notarized_chain(&third_child_id, players.as_slice()));
    }

    #[test]
    fn paper_figure_1() {
        let players = (0..100).map(|_| Id::new()).collect::<Vec<Id>>();

        let block_0 = Block::Genesis::<_, Id, _> {
            id: Id(0),
            epoch: EpochNumber(0),
        };

        let block_1 = Block::Child {
            id: Id(1),
            author: Id::new(),
            parent: block_0.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: EpochNumber(1),
        };

        let block_2 = Block::Child {
            id: Id(2),
            author: Id::new(),
            parent: block_0.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: EpochNumber(2),
        };

        let block_3 = Block::Child {
            id: Id(3),
            author: Id::new(),
            parent: block_1.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: EpochNumber(3),
        };

        let block_4 = Block::Child {
            id: Id(4),
            author: Id::new(),
            parent: block_3.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: EpochNumber(4),
        };

        let block_5 = Block::Child {
            id: Id(5),
            author: Id::new(),
            parent: block_2.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: EpochNumber(5),
        };

        let block_6 = Block::Child {
            id: Id(6),
            author: Id::new(),
            parent: block_5.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: EpochNumber(6),
        };

        let block_7 = Block::Child {
            id: Id(7),
            author: Id::new(),
            parent: block_6.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: EpochNumber(7),
        };

        let p = crate::pool::Pool::from_iter(
            vec![
                &block_0, &block_1, &block_2, &block_3, &block_4, &block_5, &block_6, &block_7,
            ]
            .into_iter()
            .cloned(),
        );

        // Genesis is finalized.
        assert_eq!(
            Some(block_0.get_id()),
            p.get_finalized(block_0.get_id(), &players)
        );

        // Block 1 is not finalized.
        assert_eq!(
            Some(block_0.get_id()),
            p.get_finalized(block_1.get_id(), &players)
        );

        // Block 2 is not finalized from the perspective of itself.
        assert_eq!(
            Some(block_0.get_id()),
            p.get_finalized(block_2.get_id(), &players)
        );

        // Block 3 is not finalized.
        assert_eq!(
            Some(block_0.get_id()),
            p.get_finalized(block_3.get_id(), &players)
        );

        // Block 4 is not finalized.
        assert_eq!(
            Some(block_0.get_id()),
            p.get_finalized(block_4.get_id(), &players)
        );

        // Block 5 is not finalized from the perspective of itself.
        assert_eq!(
            Some(block_0.get_id()),
            p.get_finalized(block_5.get_id(), &players)
        );

        // Block 6 is not finalized from the perspective of itself.
        assert_eq!(
            Some(block_0.get_id()),
            p.get_finalized(block_6.get_id(), &players)
        );

        // Block 7 is not finalized, but block 6 is from the perspective of block 7.
        assert_eq!(
            Some(block_6.get_id()),
            p.get_finalized(block_7.get_id(), &players)
        );
    }
}
