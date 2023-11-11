# A workout parser

Ever wanted to parse your workout data from a file? Well, now you can! Take a look at this example workout note:

```
name of the first exercise
20 x 10 this is a comment. there you write 
30 x 10 how you felt during the exercise
40 x 10 like "this was close to the edge"
50 x 10 or "this was easy, better increase the weight"
60 x 10 the first number is the weight, the second is the reps
70 x 10 + 40 x 6 sometimes you do all you can with one weight and then immediately you take a smaller weight and do a few more reps

bench press
20 x 10
50 x 10
60 x 10 near-death experience
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
