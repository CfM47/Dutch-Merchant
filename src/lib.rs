#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod evaluator;
pub mod model;
pub mod solver;


pub use model::instance::Instance;

mod generator;

#[cfg(feature = "python")]
#[pyo3::pymodule]
mod dm_solution {
    use crate::model::instance::Instance;
    use crate::model::solution::Solution;
    use crate::generator::random::{RandomConfig, generate_random_instance};
    use crate::solver::brute_force::brute_force_solve;
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::evaluator::PathEvaluator;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<Instance>()?;
        m.add_class::<Solution>()?;
        m.add_class::<RandomConfig>()?;
        m.add_function(wrap_pyfunction!(generate_random_instance, m)?)?;
        m.add_function(wrap_pyfunction!(brute_force_solve, m)?)?;
        Ok(())
    }
}
