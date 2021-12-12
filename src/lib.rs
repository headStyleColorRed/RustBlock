// Modules
mod bins;
pub use bins::block::Block;
pub use bins::hashable::Hashable;
pub use bins::hashable::now;
pub use bins::blockchain::*;
pub use bins::blockValidationErr::*;
pub use bins::transaction::*;

// Utils
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn hash(&self) -> u128 {
        match &self {
            Difficulty::Easy =>   0x0000fffffffffffffffffffffffffffff,
            Difficulty::Medium => 0x00000ffffffffffffffffffffffffffff,
            Difficulty::Hard =>   0x000000fffffffffffffffffffffffffff,
        }
    }
}