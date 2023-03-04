use crate::solution::Solution;

pub trait Repair{
    fn repair(&self, solution: &mut Solution);
}