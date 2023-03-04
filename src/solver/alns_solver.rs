use crate::config::Config;
use crate::problem::Problem;
use crate::solution::Solution;

pub struct ALNSSolver{
    config: Config,
}

impl ALNSSolver {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn solve(&self, problem: &Problem) -> Solution {
        let mut solution = Solution::new();
        solution
    }
}