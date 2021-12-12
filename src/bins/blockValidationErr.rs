use std::fmt;

pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction
}

impl fmt::Debug for BlockValidationErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BlockValidationErr::MismatchedIndex => write!(f, "Block index is not correct"),
            BlockValidationErr::InvalidHash => write!(f, "Difficulty fail"),
            BlockValidationErr::AchronologicalTimestamp => write!(f, "Time did not increase between blocks"),
            BlockValidationErr::MismatchedPreviousHash => write!(f, "Current's block previous hash and previous block hash don't match"),
            BlockValidationErr::InvalidInput => write!(f, "Invalid input, not contained on outspent outputs"),
            BlockValidationErr::InsufficientInputValue => write!(f, "Insuficient input value"),
            BlockValidationErr::InvalidCoinbaseTransaction => write!(f, "Invalid coinbase transaction"),
        }
    }
}