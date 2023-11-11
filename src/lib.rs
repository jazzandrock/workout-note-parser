use pest::iterators::Pair;

#[macro_use]
extern crate pest_derive;

mod error;
pub use error::Error as ExerciseError;

#[derive(Parser)]
#[grammar = "workout.pest"]
pub struct WorkoutParser;

#[derive(Debug)]
pub struct Set {
    weight: f32,
    n_reps: i32,
}

#[derive(Debug)]
pub struct Exercise {
    name: String,
    sets: Vec<Set>,
    comment: Option<String>,
}

pub fn parse_set(name: String, pair: Pair<'_, Rule>) -> Result<Exercise, ExerciseError> {
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
                    .unwrap()
                    .as_str()
                    .trim()
                    .parse::<f32>()
                    .unwrap();
                let n_reps = pairs
                    .next()
                    .unwrap()
                    .as_str()
                    .trim()
                    .parse::<i32>()
                    .unwrap();

                sets.push(Set { weight, n_reps });

                pairella = pairs.next();
            }
            _ => return Err(ExerciseError::UnexpectedRule(pair.as_rule())),
        }
    }

    Ok(Exercise {
        name,
        sets,
        comment,
    })
}

pub fn get_exercise_from_pairs(pair: Pair<'_, Rule>) -> Result<Vec<Exercise>, ExerciseError> {
    if pair.as_rule() != Rule::exercise {
        panic!("not Found exercise");
    }

    let mut rules = pair.into_inner();
    let rule = rules.next().unwrap();
    if rule.as_rule() != Rule::name {
        panic!("not Found name");
    }
    let name = rule.as_str().to_string();
    println!("Name: {}", name);

    let mut res = Vec::new();
    while let Some(rule) = rules.next() {
        if rule.as_rule() != Rule::set {
            panic!("not Found sets");
        }

        res.push(parse_set(name.clone(), rule)?);
    }

    Ok(res)
}


#[cfg(test)]
mod tests {
    use pest::Parser;
    use super::*;

    #[test]
    fn weight() {
        let parsed = WorkoutParser::parse(Rule::weight, "35        ");
        assert!(parsed.unwrap().as_str() == "35");
    }

    #[test]
    fn set() {
        let parsed = WorkoutParser::parse(Rule::set, "35.5 x 10");
        parsed.unwrap();
    }

    #[test]
    fn extended_set() {
        let parsed = WorkoutParser::parse(Rule::set, "35.5 x 10 + 40 x 8");
        parsed.unwrap();

        let parsed = WorkoutParser::parse(Rule::set, "35.5 x 10 + 40 x 8 # this was really tough");
        parsed.unwrap();
    }

    #[test]
    fn workout() {
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

        let parsed = WorkoutParser::parse(Rule::workout, input);
        println!("Parsed: {:#?}", parsed);
        parsed.unwrap();
    }
}
