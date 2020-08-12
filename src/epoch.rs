use std::time::Instant;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Epoch(u64);

impl Epoch {
    pub fn genesis() -> Self {
        Epoch(0)
    }

    pub fn consecutive(&self) -> Self {
        Epoch(self.0 + 1)
    }
}

impl From<u64> for Epoch {
    fn from(e: u64) -> Self {
        Epoch(e)
    }
}
