pub mod route;
use self::route::Route;

#[derive(Debug, Clone)]
pub struct Solution{
    pub cost: f64,
    pub routes: Vec<Route>,
}

impl Solution{
    pub fn new() -> Solution{
        Solution{
            cost: 0.0,
            routes: Vec::new(),
        }
    }
}