use thiserror::Error;
use crate::Rule;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected rule: {0:?}")]
    UnexpectedRule(Rule),

    #[error("Not found: exercise")]
    NotFoundExercise,

    #[error("Not found: name")]
    NotFoundName,

    #[error("Not found: sets")]
    NotFoundSets,
}
