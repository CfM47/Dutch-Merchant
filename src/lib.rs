use pyo3::prelude::*;

pub mod evaluator;
pub mod model;
pub mod solver;

pub use model::instance::Instance;

mod generator;

#[pymodule]
mod dm_solution {
    #[pymodule_export]
    use super::evaluator::PathEvaluator;
}
