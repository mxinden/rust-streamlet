use crate::player::PlayerId as PlayerIdT;

pub trait EpochNumber {
    fn genesis() -> Self;
    fn consecutive(&self) -> Self;
}

pub struct Epoch<Number: EpochNumber, PlayerId: PlayerIdT> {
    number: Number,
    leader: PlayerId,
}
