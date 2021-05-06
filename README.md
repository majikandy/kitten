# Kitten

## Installation
  `cargo install kitten`
 
**Kitten** is a very light acceptance test framework for Rust. It provides things that teams may prefer over cucumber or other bdd frameworks.

- Use default language features
- Keep tests as simple and clean as possible

## Show me the code

```
use kitten::*;

#[test]
fn calculator_can_add_numbers() {
    Test::scenario()
        .given(a_calculator)
        .when(adding_1_and_2)
        .then(the_answer_is_3);
    }
}

fn a_calculator() -> Calculator {
    Calculator {}
}

fn adding_1_and_2(calculator: &Calculator) -> i32 {
    calculator.add(1, 2)
}

fn the_answer_is_3(the_answer: &i32) -> () {
    assert_eq!(3, the_answer.clone());
}
```

In the above, the given/when/then accept functions where the given returns the system under test, the when takes the system under test as a parameter and returns the result which is fed into the thens (which you can chain) to do assertions.

You could also use it like this with closures...
```
 Test::scenario()
    .given(|| Calculator {})
    .when(|calculator| calculator.add(1, 2))
    .then(|answer| assert_eq!(3, answer.clone()));
```
## How is this achieved? 

It's all Rust, and it's not driven by gherkin! It's simpler. 

With Kitten, Given When Then is all you need

A common scenario encountered is when you take a feature file with given when then in it and you find that things are worded as slightly the wrong level, there is either too much or too little detail for the automation, so you decide to change it. But changing it means changing it in every block (the scenarios), then you want to slightly reword something else and it's the same problem again. It just gets messy.

The product owner just wanted to describe the feature, they don't want all the details of all the test cases and scenarios, just confidence that it's covered.
