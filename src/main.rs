use pest::Parser;
use sportparser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"
        name of the first exercise
        20 x 10 this is a comment. there you write 
        30 x 10 how you felt during the exercise
        40 x 10 like "this was close to the edge"
        50 x 10 or "this was easy, better increase the weight"
        60 x 10 the first number is the weight, the second is the reps
        70 x 10 + 40 x 6 sometimes you do all you can with one weight and then immediately you take a smaller weight and a few more reps. You can write it as well

        bench press
        20 x 10
        50 x 10
        60 x 10 near death experience
        70 x 5 + 40 x 10   
    "#;

    let parsed = WorkoutParser::parse(Rule::workout, input)?;


    if let Some(workout) = parsed.into_iter().next() {
        if workout.as_rule() != Rule::workout {
            panic!("Expected workout");
        }

        let vec = workout
            .into_inner()
            .filter(|r| r.as_rule() == Rule::exercise)
            .map(get_exercise_from_pairs)
            .collect::<Result<Vec<_>, _>>()?;
        println!("Vec: {:#?}", vec);
    }

    Ok(())
}
