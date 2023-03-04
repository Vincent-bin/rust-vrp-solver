use crate::solution::Solution;

pub trait Destroy {
    fn destroy(&self, solution: &mut Solution);
}