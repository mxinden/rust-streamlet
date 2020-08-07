use crate::player::PlayerId as PlayerIdT;

pub trait EpochNumber {}

pub struct Epoch<Number: EpochNumber, PlayerId: PlayerIdT> {
    number: Number,
    leader: PlayerId,
}
