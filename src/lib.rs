pub struct Kitten;
pub struct GivenAnd<T> {
    data: T,
}

pub struct When<T> {
    data: T,
}

pub struct Then<T> {
    data: T,
}

use std::future::Future;

fn get_current_thread() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

impl Kitten {
    pub fn given<Result>(function: impl Fn() -> Result) -> GivenAnd<Result> {
        let result = function();
        GivenAnd { data: result }
    }

    pub fn given_async<F, Fut, Result>(f: F) -> GivenAnd<Result>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result>,
    {
        GivenAnd {
            data: get_current_thread().block_on(f()),
        }
    }
}

impl<Input> GivenAnd<Input> {
    pub fn and<Result>(self, function: impl Fn(Input) -> Result) -> GivenAnd<Result> {
        GivenAnd {
            data: function(self.data),
        }
    }

    pub fn when<Result>(self, function: impl Fn(Input) -> Result) -> When<Result> {
        When {
            data: function(self.data),
        }
    }

    pub fn and_async<F, Fut, Result>(self, f: F) -> GivenAnd<Result>
    where
        F: FnOnce(Input) -> Fut,
        Fut: Future<Output = Result>,
    {
        GivenAnd {
            data: get_current_thread().block_on(f(self.data)),
        }
    }

    pub fn when_async<F, Fut, Result>(self, f: F) -> When<Result>
    where
        F: FnOnce(Input) -> Fut,
        Fut: Future<Output = Result>,
    {
        When {
            data: get_current_thread().block_on(f(self.data)),
        }
    }
}

impl<Input> When<Input> {
    pub fn and<Result>(self, function: impl Fn(Input) -> Result) -> When<Result> {
        When {
            data: function(self.data),
        }
    }
    pub fn and_async<F, Fut, Result>(self, f: F) -> When<Result>
    where
        F: FnOnce(Input) -> Fut,
        Fut: Future<Output = Result>,
    {
        When {
            data: get_current_thread().block_on(f(self.data)),
        }
    }

    pub fn then<Result>(self, function: impl Fn(Input) -> Result) -> Then<Result> {
        Then {
            data: function(self.data),
        }
    }
    pub fn then_async<F, Fut, Result>(self, f: F) -> Then<Result>
    where
        F: FnOnce(Input) -> Fut,
        Fut: Future<Output = Result>,
    {
        Then {
            data: get_current_thread().block_on(f(self.data)),
        }
    }
}

impl<Input> Then<Input> {
    pub fn and<Result>(self, function: impl Fn(Input) -> Result) -> Then<Result> {
        Then {
            data: function(self.data),
        }
    }
    pub fn and_async<F, Fut, Result>(self, f: F) -> Then<Result>
    where
        F: FnOnce(Input) -> Fut,
        Fut: Future<Output = Result>,
    {
        Then {
            data: get_current_thread().block_on(f(self.data)),
        }
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
                1
            })
            .then(|result| {
                assert_eq!(result, 1);
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
    fn kitten_works_with_async_functions() {
        Kitten::given_async(a_calculator_async)
            .when_async(adding_1_and_2_via_a_function_async)
            .then_async(the_answer_is_3_checked_via_a_function_async);
    }

    #[test]
    fn kitten_works_with_mixing_sync_and_async_functions() {
        Kitten::given_async(a_calculator_async)
            .and(|thus_calculator| thus_calculator)
            .and_async(|thus_calculator| async { thus_calculator })
            .when_async(adding_1_and_2_via_a_function_async)
            .and(the_answer_is_3_checked_via_a_function)
            .then(|_| a_calculator())
            .and_async(adding_1_and_2_via_a_function_async)
            .and(the_answer_is_3_checked_via_a_function);
    }

    #[test]
    fn kitten_works_with_mixing_other_sync_and_async_functions() {
        Kitten::given(a_calculator)
            .and_async(|thus_calculator| async { thus_calculator })
            .when(adding_1_and_2_via_a_function)
            .and_async(the_answer_is_3_checked_via_a_function_async)
            .then(|_| a_calculator())
            .and(adding_1_and_2_via_a_function)
            .and_async(the_answer_is_3_checked_via_a_function_async);
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

    #[test]
    fn it_works_with_closures_that_capture_variables() {
        let example = "example";

        Kitten::given(|| 10)
            .and(|result| {
                assert_eq!(result, 10);
                String::from("another type")
            })
            .when(|result| {
                assert_eq!(result, String::from("another type"));
                format!("test-{}", example)
            })
            .then(|result| {
                assert_eq!(result, "test-example".to_string());
                vec!["this", "is", "a", "list"]
            })
            .and(|result| assert_eq!(result, vec!["this", "is", "a", "list"]));
    }

    fn adding_1_and_2_via_a_function(c: Calculator) -> i32 {
        c.add(1, 2)
    }

    async fn adding_1_and_2_via_a_function_async(c: Calculator) -> i32 {
        async { c.add(1, 2) }.await
    }

    fn the_answer_is_3_checked_via_a_function(the_answer: i32) {
        assert_eq!(the_answer.clone(), 3);
    }

    async fn the_answer_is_3_checked_via_a_function_async(the_answer: i32) {
        assert_eq!(the_answer.clone(), 3);
        println!("the answer is {}", the_answer);
    }

    fn a_calculator() -> Calculator {
        Calculator {}
    }

    async fn a_calculator_async() -> Calculator {
        Calculator {}
    }
}
