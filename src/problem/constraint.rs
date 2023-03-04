use crate::solution::Solution;

pub trait Constraint {
    fn is_satisfied(&self, solution: &Solution) -> bool;
}
    
