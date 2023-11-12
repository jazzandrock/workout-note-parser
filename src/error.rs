use crate::Rule;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),

    #[error("Expected rule: {0:?}")]
    ExpectedRule(Rule),

    #[error(transparent)]
    ParsingError(#[from] Box<pest::error::Error<Rule>>),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
