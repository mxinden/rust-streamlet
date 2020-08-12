pub mod block;
pub mod epoch;
pub mod player;
pub mod pool;
pub mod schedule;
pub mod simulator;

#[cfg(test)]
mod tests {
    use crate::block::{Block, BlockId as BlockIdT};
    use crate::epoch::Epoch;
    use crate::player::PlayerId;
    use crate::pool::Pool;

    use quickcheck::{Arbitrary, Gen, QuickCheck};
    use rand::prelude::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
    struct Id(u64);

    impl Id {
        fn new() -> Self {
            Id(rand::thread_rng().gen())
        }
    }

    impl BlockIdT for Id {}

    #[test]
    fn block_is_notarized_returns_true_for_genesis() {
        assert!(Block::<Id>::Genesis {
            id: Id::new(),
            epoch: Epoch::genesis(),
        }
        .is_notarized(&[]));
    }

    #[test]
    fn block_is_notarized_returns_false_for_block_with_empty_votes() {
        let players = (0..100).map(|id| id.into()).collect::<Vec<PlayerId>>();

        let genesis_epoch = Epoch::genesis();
        let genesis_block = Block::Genesis::<Id> {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let child_block = Block::Child {
            id: Id::new(),
            author: 0.into(),
            parent: genesis_block.get_id().clone(),
            votes: vec![],
            epoch: genesis_epoch.consecutive(),
        };

        assert!(!child_block.is_notarized(players.as_slice()));
    }

    #[test]
    fn block_with_two_thirds_is_notarized_returns_true() {
        let players = (0..100).map(|id| id.into()).collect::<Vec<PlayerId>>();

        let genesis_epoch = Epoch::genesis();
        let genesis_block = Block::Genesis::<Id> {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let child_block = Block::Child {
            id: Id::new(),
            author: 0.into(),
            parent: genesis_block.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        assert!(child_block.is_notarized(players.as_slice()));
    }

    #[test]
    fn is_notarized_chain_all_blocks_notarized() {
        let players = (0..100).map(|id| id.into()).collect::<Vec<PlayerId>>();

        let genesis_epoch = Epoch::genesis();
        let genesis_block = Block::Genesis::<Id> {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let first_child = Block::Child {
            id: Id::new(),
            author: 0.into(),
            parent: genesis_block.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        let second_child_id = Id::new();
        let second_child = Block::Child {
            id: second_child_id.clone(),
            author: 0.into(),
            parent: first_child.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        let p = Pool::from_iter(vec![genesis_block, first_child, second_child]);

        assert!(p.is_notarized_chain(&second_child_id, players.as_slice()));
    }

    #[test]
    fn is_notarized_chain_one_block_not_notarized() {
        let players = (0..100).map(|id| id.into()).collect::<Vec<PlayerId>>();

        let genesis_epoch = Epoch::genesis();
        let genesis_block = Block::Genesis::<Id> {
            id: Id::new(),
            epoch: genesis_epoch,
        };

        let first_child = Block::Child {
            id: Id::new(),
            author: 0.into(),
            parent: genesis_block.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        // Not notarized.
        let second_child = Block::Child {
            id: Id::new(),
            author: 0.into(),
            parent: first_child.get_id().clone(),
            votes: players.iter().take(60).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        let third_child_id = Id::new();
        let third_child = Block::Child {
            id: third_child_id.clone(),
            author: 0.into(),
            parent: second_child.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: genesis_epoch.consecutive(),
        };

        let p = Pool::from_iter(vec![genesis_block, first_child, second_child, third_child]);

        assert!(!p.is_notarized_chain(&third_child_id, players.as_slice()));
    }

    #[test]
    fn paper_figure_1() {
        let players = (0..100).map(|id| id.into()).collect::<Vec<PlayerId>>();

        let block_0 = Block::Genesis::<_> {
            id: Id(0),
            epoch: Epoch::genesis(),
        };

        let block_1 = Block::Child {
            id: Id(1),
            author: 0.into(),
            parent: block_0.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: 1.into(),
        };

        let block_2 = Block::Child {
            id: Id(2),
            author: 0.into(),
            parent: block_0.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: 2.into(),
        };

        let block_3 = Block::Child {
            id: Id(3),
            author: 0.into(),
            parent: block_1.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: 3.into(),
        };

        let block_4 = Block::Child {
            id: Id(4),
            author: 0.into(),
            parent: block_3.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: 4.into(),
        };

        let block_5 = Block::Child {
            id: Id(5),
            author: 0.into(),
            parent: block_2.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: 5.into(),
        };

        let block_6 = Block::Child {
            id: Id(6),
            author: 0.into(),
            parent: block_5.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: 6.into(),
        };

        let block_7 = Block::Child {
            id: Id(7),
            author: 0.into(),
            parent: block_6.get_id().clone(),
            votes: players.iter().take(66).cloned().collect(),
            epoch: 7.into(),
        };

        let p = Pool::from_iter(
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

    #[derive(Clone, Debug)]
    struct NotarizedPool<BlockId>(Pool<BlockId>);

    impl Arbitrary for NotarizedPool<Id> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut max_epoch = Epoch::genesis();
            let mut p = Pool::from_iter(vec![]);
            let players = (0..100).map(|id| id.into()).collect::<Vec<PlayerId>>();

            let mut heads = std::collections::VecDeque::new();
            heads.push_back(Block::Genesis::<_> {
                id: Id(0),
                epoch: max_epoch,
            });

            while let Some(head) = heads.pop_front() {
                if max_epoch > 100.into() {
                    break;
                }

                for _ in 0..g.gen_range(0, 5) {
                    max_epoch = max_epoch.consecutive();
                    heads.push_back(Block::Child {
                        id: Id::new(),
                        author: 0.into(),
                        parent: *head.get_id(),
                        votes: players.iter().take(66).cloned().collect(),
                        epoch: max_epoch,
                    })
                }

                p.insert(head);
            }

            NotarizedPool(p)
        }
    }

    #[test]
    fn there_can_never_be_two_finalized_chains() {
        fn prop(pool: NotarizedPool<Id>) {
            let pool = pool.0;

            // TODO: These are not the same players as used to construct the pool.
            let players = (0..100).map(|id| id.into()).collect::<Vec<PlayerId>>();

            let mut finalized_blocks = pool
                .iter_blocks()
                .map(|block_id| pool.get_finalized(block_id, &players))
                .filter_map(|block_id| Some((pool.get_epoch(block_id?)?, block_id?)))
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            finalized_blocks.sort_by(|a, b| a.0.cmp(&b.0));

            let youngest_block = match finalized_blocks.pop() {
                Some(id) => id.1,
                None => return,
            };

            for (_epoch, older_block) in finalized_blocks {
                assert!(
                    pool.chain_contains(youngest_block, older_block),
                    "Expect all older blocks to be part of the chain of the \
                     youngest finalized block to ensure there to be at most \
                     one finalized chain.",
                );
            }
        }

        QuickCheck::new().quickcheck(prop as fn(_) -> _)
    }
}
