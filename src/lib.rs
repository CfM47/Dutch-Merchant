#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod evaluator;
pub mod model;
pub mod solver;

#[cfg(feature = "python")]
#[pymodule]
mod dm_solution {
    #[pymodule_export]
    use super::evaluator::PathEvaluator;
}
