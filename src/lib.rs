use pyo3::prelude::*;

pub mod evaluator;
pub mod model;
pub mod solver;

pub use model::instance::Instance;

mod generator;

#[pymodule]
mod dm_solution {
    use crate::generator;

    #[pymodule_export]
    use super::evaluator::PathEvaluator;

    #[pymodule_export]
    use generator::generate_instance;
}
