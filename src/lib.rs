pub struct Kitten;

impl Kitten {
    pub fn given<Result>(function: fn() -> Result) -> GivenAnd<Result> {
        let result = function();
        GivenAnd { data: result }
    }
}

pub struct GivenAnd<T> {
    data: T,
}

impl<Input> GivenAnd<Input> {
    pub fn and<Result>(self, function: fn(Input) -> Result) -> GivenAnd<Result> {
        let result = function(self.data);
        GivenAnd { data: result }
    }

    pub fn when<Result>(self, function: fn(Input) -> Result) -> When<Result> {
        let result = function(self.data);
        When { data: result }
    }
}

pub struct When<T> {
    data: T,
}

impl<Input> When<Input> {
    pub fn then<Result>(self, function: fn(Input) -> Result) -> Then<Result> {
        let result = function(self.data);
        Then { data: result }
    }
}

pub struct Then<T> {
    data: T,
}

impl<Input> Then<Input> {
    pub fn and<Result>(self, function: fn(Input) -> Result) -> Then<Result> {
        let result = function(self.data);
        Then { data: result }
    }
}
#[cfg(test)]
mod tests {
    mod calculator;
    use crate::Kitten;
    use calculator::Calculator;

    use self::calculator::Calc;

    #[test]
    fn kitten_works_with_closures() {
        Kitten::given(|| 10)
            .and(|result| {
                assert_eq!(result, 10);
                String::from("another type")
            })
            .when(|result| {
                assert_eq!(result, String::from("another type"));
                ()
            })
            .then(|result| {
                assert_eq!(result, ());
                vec!["this", "is", "a", "list"]
            })
            .and(|result| assert_eq!(result, vec!["this", "is", "a", "list"]));
    }

    #[test]
    fn kitten_works_with_named_functions_and_looks_clean() {
        Kitten::given(a_calculator)
            .when(adding_1_and_2_via_a_function)
            .then(the_answer_is_3_checked_via_a_function);
    }
    #[test]

    fn it_works_with_named_functions_closures_and_closures_that_call_functions() {
        Kitten::given(a_calculator) // last return in given chain passed to when
            .when(adding_1_and_2_via_a_function)
            .then(|answer| {
                assert_eq!(answer, 3);
                answer
            })
            .and(the_answer_is_3_checked_via_a_function);
    }

    fn adding_1_and_2_via_a_function(c: Calculator) -> i32 {
        c.add(1, 2)
    }

    fn the_answer_is_3_checked_via_a_function(the_answer: i32) -> () {
        assert_eq!(the_answer.clone(), 3);
    }

    fn a_calculator() -> Calculator {
        Calculator {}
    }
}
