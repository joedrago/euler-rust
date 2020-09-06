mod problem;
use problem::Problem;

mod problem_001;
pub use problem_001::Problem001;
mod problem_002;
pub use problem_002::Problem002;

pub fn all() -> Vec<Box<dyn Problem>> {
    let mut all_problems: Vec<Box<dyn Problem>> = Vec::new();
    all_problems.push(Box::new(Problem001 {}));
    all_problems.push(Box::new(Problem002 {}));
    return all_problems;
}
