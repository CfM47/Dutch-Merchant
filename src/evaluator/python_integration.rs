use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{
    evaluator::{
        InfiniteCapacityDebtEvaluator, IntervalEvaluator, LpProfitCalculator,
        path_evaluator::PathEvaluator as PathEvaluatorTrait,
    },
    model::instance::Instance,
};

#[pyclass]
pub struct PathEvaluator {
    problem_instance: Instance,
    interval_evaluator: IntervalEvaluator,
    lp_evaluator: LpProfitCalculator,
    inf_capacity_debt_evaluator: InfiniteCapacityDebtEvaluator,
}

#[pymethods]
impl PathEvaluator {
    #[new]
    pub fn new(instance_data: &str) -> Self {
        let problem_instance: Instance =
            serde_json::from_str(instance_data).expect("Failed to parse instance data");

        PathEvaluator {
            problem_instance,
            interval_evaluator: IntervalEvaluator::new(),
            lp_evaluator: LpProfitCalculator::new(),
            inf_capacity_debt_evaluator: InfiniteCapacityDebtEvaluator::new(),
        }
    }

    pub fn score_route(
        &self,
        path: Vec<usize>,
        evaluator_name: &str,
    ) -> PyResult<(f64, Vec<Vec<(f64, f64)>>)> {
        let evaluator: &dyn PathEvaluatorTrait = if evaluator_name == self.interval_evaluator.name()
        {
            &self.interval_evaluator
        } else if evaluator_name == self.lp_evaluator.name() {
            &self.lp_evaluator
        } else if evaluator_name == self.inf_capacity_debt_evaluator.name() {
            &self.inf_capacity_debt_evaluator
        } else {
            return Err(PyValueError::new_err("Unsupported evaluator name"));
        };

        Ok(evaluator.calculate_best_profit(&self.problem_instance, &path))
    }
}
