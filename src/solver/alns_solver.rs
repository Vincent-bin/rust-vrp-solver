use crate::algorithm::destroy::{self, Destroy};
use crate::algorithm::repair::Repair;
use crate::config::Config;
use crate::problem::constraint::Constraint;
use crate::problem::Problem;
use crate::solution::Solution;

pub struct ALNSSolver {
    config: Config,
    destroyers: Vec<Box<dyn Destroy>>,
    repairers: Vec<Box<dyn Repair>>
}

impl ALNSSolver {
    pub fn new(config: Config) -> Self {
        Self { config, destroyers: todo!(), repairers: todo!() }
    }

    pub fn solve(&self, problem: &Problem) -> Solution {
        let mut solution = Solution::new();
        let mut best_solution = Solution::new();
        let constraint = &problem.constraints;

        for _ in 0..self.config.max_iterations {
            let destroyer = self.select_destroyer(&solution);
            destroyer.destroy(&mut solution);
            let repairer = self.select_repairer(&solution);
            repairer.repair(&mut solution);
            
            let cost = self.evaluate(&solution, &problem);
            solution.cost = cost;
            if self.accept(&solution, constraint) {
                self.update_best_solution(&solution, &mut best_solution);
            }
        }
        best_solution
    }

    fn accept(&self, solution: &Solution, constraint: &Vec<Box<dyn Constraint>>) -> bool {
        for c in constraint {
            if !c.is_satisfied(solution) {
                return false;
            }
        }
        true
    }

     fn select_destroyer(&self, solution: &Solution) -> Box<dyn Destroy> {
        todo!()
    }

     fn select_repairer(&self, solution: &Solution) -> Box<dyn Repair> {
        todo!()
    }

    fn update_best_solution(&self, solution: &Solution, best_solution: &mut Solution) {
        if solution.cost < best_solution.cost {
            *best_solution = solution.clone();
        }
    }

    fn evaluate(&self, solution: &Solution, problem: &&Problem) -> f64 {
        todo!()
    }

}
