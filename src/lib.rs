pub trait GWT<T, SUT> {
    fn given(self, function: fn() -> SUT) -> Self;
    fn and_given(self, function: fn() -> SUT) -> Self;
    fn when(self, function: fn(&SUT) -> T) -> Self;
    fn then(self, function: fn(&T) -> &T) -> Self;
    fn and(self, function: fn(&T) -> &T) -> Self;
}
pub struct Test<T, SUT> {
    pub system_under_test: Option<SUT>,
    pub result: Option<T>,
}

impl<T, SUT> Test<T, SUT> {
    pub fn scenario() -> Self {
        Test::<T, SUT> {
            result: None,
            system_under_test: None,
        }
    }
}

impl<T, SUT> GWT<T, SUT> for Test<T, SUT> {
    fn given(mut self, function: fn() -> SUT) -> Self {
        self.system_under_test = Some(function());
        return self;
    }
    fn and_given(mut self, function: fn() -> SUT) -> Self {
        self.system_under_test = Some(function());
        return self;
    }
    fn when(mut self, function: fn(&SUT) -> T) -> Self {
        self.result = Some(function(
            self.system_under_test
                .as_ref()
                .expect("SUT should not be None"),
        ));
        return self;
    }
    fn then(self, function: fn(&T) -> &T) -> Self {
        function(self.result.as_ref().expect("Result should not be None"));
        return self;
    }
    fn and(self, function: fn(&T) -> &T) -> Self {
        function(self.result.as_ref().expect("Result should not be None"));
        return self;
    }
}
mod calculator;
#[cfg(test)]
mod tests {
    use super::*;
    use calculator::*;

    #[test]
    fn it_works_with_closures() {
        Test::scenario()
            .given(|| ())
            .when(|_sut| "something is returned by system under test")
            .then(|x| {
                assert_eq!(x.clone(), "something is returned by system under test");
                x
            })
            .and(|x| {
                assert_eq!(1, 1);
                x
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
            .given(a_calculator)
            .and_given(a_calculator)
            .when(adding_1_and_2_via_a_function)
            .when(|c| adding(c, 1, 2))
            .then(|answer| {
                assert_eq!(answer.clone(), 3);
                answer
            })
            .and(the_answer_is_3_checked_via_a_function);
    }

    fn adding_1_and_2_via_a_function(c: &Calculator) -> i32 {
        c.add(1, 2)
    }

    fn the_answer_is_3_checked_via_a_function(the_answer: &i32) -> &i32 {
        assert_eq!(the_answer.clone(), 3);
        the_answer
    }

    fn adding(calculator: &Calculator, left: i32, right: i32) -> i32 {
        calculator.add(left, right)
    }

    fn a_calculator() -> Calculator {
        Calculator {}
    }
}