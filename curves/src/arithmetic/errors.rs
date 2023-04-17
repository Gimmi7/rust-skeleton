use std::error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParseBigIntError {
    pub reason: ParseBigIntErrorReason,
    pub radix: u32,
}

#[derive(Debug)]
pub enum ParseBigIntErrorReason {
    NumBigint
}

impl Display for ParseBigIntError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.reason {
            ParseBigIntErrorReason::NumBigint => {
                write!(f, "num-bigint: invalid {}-based number representation", self.radix)
            }
        }
    }
}

impl error::Error for ParseBigIntError {}