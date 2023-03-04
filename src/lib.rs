pub mod problem;
pub mod solution;
pub mod solver;
pub mod config;
pub mod algorithm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
