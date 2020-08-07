mod block;
mod epoch;
mod player;
mod vote;

#[cfg(test)]
mod tests {
    use crate::block::{Block, BlockId};
    use crate::epoch::EpochNumber as EpochNumberT;
    use crate::player::PlayerId;

    use rand::prelude::*;

    struct Id(u64);

    impl Id {
        fn new() -> Self {
            Id(rand::thread_rng().gen())
        }
    }

    impl BlockId for Id {}

    impl PlayerId for Id {}

    struct EpochNumber(u64);

    impl EpochNumber {
        fn genesis() -> Self {
            EpochNumber(0)
        }
    }

    impl EpochNumberT for EpochNumber {}

    #[test]
    fn block_is_notarized_returns_true_for_genesis() {
        assert!(Block::<Id, Id, EpochNumber>::Genesis { id: Id::new() }.is_notarized());
    }
}
