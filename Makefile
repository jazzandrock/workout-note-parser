.PHONY: run run-stdin run-stdout run-file fmt check all

# Literal text block
define EXERCISE_INPUT
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
endef
export EXERCISE_INPUT


# Default command to build and check everything
all: check run

# Run the app with the literal text as standard input
run:
	@echo "$$EXERCISE_INPUT" | cargo run -q -- -i -

# Format code using rustfmt
check:
	cargo fmt -- --check
	cargo clippy -- -D warnings
