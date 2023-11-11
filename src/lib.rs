#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "workout.pest"] // path to your .pest file
struct WorkoutParser;

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
