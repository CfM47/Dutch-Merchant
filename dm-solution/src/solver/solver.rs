use crate::model::{instance::Instance, solution::Solution};

/// A solver for the DM problem
pub trait Solver {
    /// The name of the solver (e.g. "greedy")
    fn name(&self) -> &'static str;

    fn solve(&self, instance: &Instance) -> Option<Solution>;
}
