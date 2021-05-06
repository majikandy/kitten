pub trait GWT<Result, SystemUnderTest> {
    fn given(self, function: fn() -> SystemUnderTest) -> Self;
    fn and_given(self, function: fn() -> SystemUnderTest) -> Self;
    fn when(self, function: fn(&SystemUnderTest) -> Result) -> Self;
    fn then(self, function: fn(&Result) -> ()) -> Self;
    fn and(self, function: fn(&Result) -> ()) -> Self;
}
pub struct Test<Result, SystemUnderTest> {
    pub system_under_test: Option<SystemUnderTest>,
    pub result: Option<Result>,
}

impl<Result, SystemUnderTest> Test<Result, SystemUnderTest> {
    pub fn scenario() -> Self {
        Test::<Result, SystemUnderTest> {
            result: None,
            system_under_test: None,
        }
    }
}

impl<Result, SystemUnderTest> GWT<Result, SystemUnderTest> for Test<Result, SystemUnderTest> {
    fn given(mut self, function: fn() -> SystemUnderTest) -> Self {
        self.system_under_test = Some(function());
        return self;
    }
    fn and_given(mut self, function: fn() -> SystemUnderTest) -> Self {
        self.system_under_test = Some(function());
        return self;
    }
    fn when(mut self, function: fn(&SystemUnderTest) -> Result) -> Self {
        self.result =
            Some(function(self.system_under_test.as_ref().expect(
                "SYSTEM_UNDER_TEST should not be None, you must have a given",
            )));
        return self;
    }
    fn then(self, function: fn(&Result) -> ()) -> Self {
        function(self.result.as_ref().expect("Result should not be None"));
        return self;
    }
    fn and(self, function: fn(&Result) -> ()) -> Self {
        function(self.result.as_ref().expect("Result should not be None"));
        return self;
    }
}

#[cfg(test)]
mod tests {
    mod calculator;
    use super::*;
    use calculator::*;

    #[test]
    fn it_works_with_closures() {
        Test::scenario()
            .given(|| ())
            .when(|_| "something is returned by system under test")
            .then(|x| {
                assert_eq!(
                    x.clone(),
                    "something is returned by system under test",
                    "comparisons can be checked against the return"
                );
            })
            .then(|x| {
                assert!(!x.clone().is_empty(), "chaining asserts :)");
            })
            .and(|_| {
                assert_eq!(1, 1);
            });
    }

    #[test]
    fn it_works_with_named_functions_and_looks_clean() {
        Test::scenario()
            .given(a_calculator)
            .when(adding_1_and_2_via_a_function)
            .then(the_answer_is_3_checked_via_a_function);
    }

    #[test]
    fn it_works_with_named_functions_closures_and_closures_that_call_functions() {
        Test::scenario()
            .given(|| Calculator {})
            .and_given(a_calculator) // last return in given chain passed to when
            .when(adding_1_and_2_via_a_function)
            .when(|c| adding(c, 1, 2))
            .then(|answer| {
                assert_eq!(answer.clone(), 3);
            })
            .and(the_answer_is_3_checked_via_a_function);
    }

    fn adding_1_and_2_via_a_function(c: &Calculator) -> i32 {
        c.add(1, 2)
    }

    fn the_answer_is_3_checked_via_a_function(the_answer: &i32) -> () {
        assert_eq!(the_answer.clone(), 3);
    }

    fn adding(calculator: &Calculator, left: i32, right: i32) -> i32 {
        calculator.add(left, right)
    }

    fn a_calculator() -> Calculator {
        Calculator {}
    }
}
