use crate::Rule;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),

    #[error("Expected rule: {0:?}")]
    ExpectedRule(Rule),

    #[error("parsing error: {0}")]
    ParsingError(#[from] pest::error::Error<Rule>),

    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
