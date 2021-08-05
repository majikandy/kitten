# Kitten

## Installation
  `cargo install kitten`

**Kitten** is a very light acceptance test framework for Rust. It provides things that teams may prefer over cucumber or other bdd frameworks.

- Use default language features
- Keep tests as simple and clean as possible

## Show me the code

```rust
use kitten::*;

#[test]
fn calculator_can_add_numbers() {
    Kitten::given(a_calculator)
        .when(adding_1_and_2)
        .then(the_answer_is_3);
    }
}

fn a_calculator() -> Calculator {
    Calculator {}
}

fn adding_1_and_2(calculator: Calculator) -> i32 {
    calculator.add(1, 2)
}

fn the_answer_is_3(the_answer: i32) -> () {
    assert_eq!(3, the_answer);
}
```

In the above, the flow of `.given/.when/.then/.and` acts like a chain, where each step accepts a function that feeds the result into the input of the succeeding one.

You could also use it like this with closures...

```rust
Kitten::given(|| Calculator {})
    .when(|calculator| calculator.add(1, 2))
    .then(|answer| assert_eq!(3, answer.clone()));
```

The function of a step has a generic return type, so you are free to adapt what is passed through the chain, like so:

```rust
Kitten::given(|| "hello world") // we return a &str
    .when(|hello_world| Some(hello_world)) // we return a Option<&str>
    .then(|some_hello_word| {
        assert_eq!(some_hello_world.is_none(), false); // we assert on our Option<&str>
    });
```

This feature also comes in handy when you need to pass results in-between steps:

```rust
Kitten::given(|| Foo{})
    .and(|foo| (foo,  Bar{}))
    .when(|(foo, bar)| {
        foo.update();
        bar.update();
        (foo, bar)
    })
    .then(|(foo, bar)| {
        foo_is_updated(foo);
        bar_is_updated(bar);
    });
```

## How is this achieved?

It's all Rust, and it's not driven by gherkin! It's simpler.

With Kitten, Given When Then is all you need

A common scenario encountered is when you take a feature file with given when then in it and you find that things are worded as slightly the wrong level, there is either too much or too little detail for the automation, so you decide to change it. But changing it means changing it in every block (the scenarios), then you want to slightly reword something else and it's the same problem again. It just gets messy.

The product owner just wanted to describe the feature, they don't want all the details of all the test cases and scenarios, just confidence that it's covered.
