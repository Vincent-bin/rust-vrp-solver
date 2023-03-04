pub mod vehicle;
pub mod job;
pub mod constraint;
pub mod objective;


pub struct Problem {
    pub vehicles: Vec<vehicle::Vehicle>,
    pub jobs: Vec<job::Job>,
    pub constraints: Vec<Box<dyn constraint::Constraint>>,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            vehicles: Vec::new(),
            jobs: Vec::new(),
            constraints: Vec::new(),
        }
    }
}