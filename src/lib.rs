use pest::{iterators::Pair, Parser};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate pest_derive;

pub mod error;
use error::Error;

#[derive(Parser)]
#[grammar = "workout.pest"]
pub struct WorkoutParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    weight: f32,
    n_reps: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    name: String,
    sets: Vec<Set>,
    comment: Option<String>,
}

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
