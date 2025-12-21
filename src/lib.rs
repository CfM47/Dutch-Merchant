use pyo3::prelude::*;

pub mod evaluator;
pub mod model;
pub mod solver;

#[pymodule]
mod dm_solution {
    #[pymodule_export]
    use super::evaluator::PathEvaluator;
}
