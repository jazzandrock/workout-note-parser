//! This module contains the implementation of a parser for workout data.
//!
//! The parser is implemented using the pest parser generator and is used to parse
//! workout data in a custom format. The parsed data is then used to generate
//! workout plans and track progress.

use pest::{iterators::Pair, Parser};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate pest_derive;

pub mod error;
use error::Error;

#[derive(Parser)]
#[grammar = "workout.pest"]
struct WorkoutParser;

/// Represents a set in weightlifting, consisting of a weight and the number of repetitions performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    /// The weight used for this set.
    pub weight: f32,
    /// The number of repetitions performed for this set.
    pub n_reps: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A struct representing an exercise, actually, a single set
pub struct Exercise {
    // the name of your exercise
    pub name: String,
    // this is a vector because you can do two sets without a rest, and this counts as a single set
    pub sets: Vec<Set>,
    // a comment about the exercise
    pub comment: Option<String>,
}

/// Parses a set of exercises from a `Pair` object.
///
/// # Arguments
///
/// * `name` - A `String` representing the name of the exercise.
/// * `pair` - A `Pair` object representing the set of exercises to parse.
///
/// # Returns
///
/// Returns a `Result` containing an `Exercise` object if parsing was successful, or an `Error` if parsing failed.
pub fn parse_set(name: String, pair: Pair<'_, Rule>) -> Result<Exercise, Error> {
    if pair.as_rule() != Rule::set {
        return Err(Error::ExpectedRule(Rule::set));
    }
    let mut pairella = Some(pair);

    let mut sets = Vec::new();
    let mut comment = None;
    while let Some(pair) = pairella {
        match pair.as_rule() {
            Rule::comment => {
                comment = Some(pair.as_str().trim().to_string());
                pairella = pair.into_inner().next();
            }
            Rule::set => {
                let mut pairs = pair.into_inner();
                let weight = pairs
                    .next()
                    .ok_or(Error::ExpectedRule(Rule::weight))?
                    .as_str()
                    .trim()
                    .parse::<f32>()?;
                let n_reps = pairs
                    .next()
                    .ok_or(Error::ExpectedRule(Rule::reps))?
                    .as_str()
                    .trim()
                    .parse::<i32>()?;

                sets.push(Set { weight, n_reps });

                pairella = pairs.next();
            }
            _ => return Err(Error::UnexpectedRule(pair.as_rule())),
        }
    }

    Ok(Exercise {
        name,
        sets,
        comment,
    })
}

/// Parses an exercise from a `Pair` and returns a vector of `Exercise`s.
///
/// # Arguments
///
/// * `pair` - A `Pair` that represents an exercise.
///
/// # Returns
///
/// A `Result` containing a vector of `Exercise`s if parsing was successful, or an `Error` if parsing failed.
pub fn get_exercise_from_pairs(pair: Pair<'_, Rule>) -> Result<Vec<Exercise>, Error> {
    if pair.as_rule() != Rule::exercise {
        return Err(Error::ExpectedRule(Rule::exercise));
    }

    let mut rules = pair.into_inner();
    let rule = rules.next().ok_or(Error::ExpectedRule(Rule::name))?;
    if rule.as_rule() != Rule::name {
        return Err(Error::ExpectedRule(Rule::name));
    }

    let name = rule.as_str().to_string();

    rules
        .filter(|r| r.as_rule() == Rule::set)
        .map(|r| parse_set(name.clone(), r))
        .collect()
}

/// Parses a workout input string and returns a vector of exercises.
///
/// # Arguments
///
/// * `input` - A string slice that holds the workout input.
///
/// # Returns
///
/// * `Result<Vec<Exercise>, Error>` - A `Result` that holds a vector of `Exercise`s if parsing was successful, or an `Error` if parsing failed.
pub fn parse_workout(input: &str) -> Result<Vec<Exercise>, Error> {
    let mut parsed = WorkoutParser::parse(Rule::workout, input)?;

    let workout = parsed.next().ok_or(Error::ExpectedRule(Rule::workout))?;
    if workout.as_rule() != Rule::workout {
        return Err(Error::ExpectedRule(Rule::workout));
    }

    let vec = workout
        .into_inner()
        .filter(|r| r.as_rule() == Rule::exercise)
        .map(get_exercise_from_pairs)
        .collect::<Result<Vec<_>, _>>()?;
    let vec = vec.into_iter().flatten().collect::<Vec<_>>();

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;
    use anyhow::Result;


    #[test]
    fn weight() -> Result<()> {
        let parsed = WorkoutParser::parse(Rule::weight, "35        ")?;
        assert!(parsed.as_str() == "35");

        Ok(())
    }

    #[test]
    fn set() -> Result<()> {
        WorkoutParser::parse(Rule::set, "35.5 x 10")?;

        Ok(())
    }

    #[test]
    fn extended_set() -> Result<()> {
        WorkoutParser::parse(Rule::set, "35.5 x 10 + 40 x 8")?;

        WorkoutParser::parse(Rule::set, "35.5 x 10 + 40 x 8 # this was really tough")?;

        Ok(())
    }

    #[test]
    fn workout() -> Result<()> {
        let input = r#"
        vert block
        35.5 x 10
        35.5 x 10 + 40 x 8
        35.5 x 10 + 40 x 8 this was really tough
    
        vert block
        35.5 x 10
        35.5 x 10 + 40 x 8
        35.5 x 10 + 40 x 8 this was really tough

        vert block
        35.5 x 10
        35.5 x 10 + 40 x 8
        35.5 x 10 + 40 x 8 this was really tough
        "#;

        let parsed = WorkoutParser::parse(Rule::workout, input)?;
        println!("Parsed: {:#?}", parsed);

        Ok(())
    }
}
