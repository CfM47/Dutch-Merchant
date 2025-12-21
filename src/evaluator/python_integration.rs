use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{
    evaluator::{IntervalSolver, path_evaluator::PathEvaluator as PathEvaluatorTrait},
    model::instance::Instance,
};

#[pyclass]
pub struct PathEvaluator {
    problem_instance: Instance,
    interval_evaluator: IntervalSolver,
}

#[pymethods]
impl PathEvaluator {
    #[new]
    pub fn new(instance_data: &str) -> Self {
        let interval_evaluator = IntervalSolver::new();
        let problem_instance: Instance =
            serde_json::from_str(instance_data).expect("Failed to parse instance data");

        PathEvaluator {
            problem_instance,
            interval_evaluator,
        }
    }

    pub fn score_route(
        &self,
        path: Vec<usize>,
        evaluator_name: &str,
    ) -> PyResult<(f64, Vec<Vec<f64>>)> {
        if evaluator_name == self.interval_evaluator.name() {
            return Ok(self
                .interval_evaluator
                .calculate_best_profit(&self.problem_instance, &path));
        }

        Err(PyValueError::new_err("Unsupported evaluator name"))
    }
}
