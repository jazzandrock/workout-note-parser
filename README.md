# Workout note parser

[![crate](https://img.shields.io/badge/crates.io-workout--note--parser-orange)](https://crates.io/crates/workout-note-parser)

Ever wanted to parse your workout data from a file? Well, now you can! Take a look at this example workout note:

```
Name of the first exercise
20 x 10 This is a comment. There you write 
30 x 10 how you felt during the exercise,
40 x 10 like "this was close to the edge"
50 x 10 or "this was easy, better increase the weight".
60 x 10 The first number is the weight, the second is the number of reps.
70 x 10 + 40 x 6 Sometimes you do all you can with one weight and then 
80 x 10 immediately you take a smaller weight and do a few more reps. 
90 x 10 You can write it as well

bench press
20 x 10
50 x 10
60 x 10 near death experience
70 x 5 + 40 x 10   
```

Now you can parse it down to a neat data structure:

```rust
struct Set {
    weight: f32,
    n_reps: i32,
}

struct Exercise {
    name: String,
    sets: Vec<Set>,
    comment: Option<String>,
}
```

# Usage

Use the command this way:
```bash
cargo install workout-note-parser

workout-note-parser -i - < input.txt > output.txt
workout-note-parser -i input.txt > output.txt
workout-note-parser -i input.txt -o output.txt
```

You can then use the output (in JSON format) to plot your progress, for example, with ChatGPT data analysis tool. Or just write a plotter yourself.

# License

MIT license
