pub trait Calc {
    fn add(&self, left: i32, right: i32) -> i32;
}
pub(crate) struct Calculator;

impl Calc for Calculator {
    fn add(&self, left: i32, right: i32) -> i32 {
        left + right
    }
}
