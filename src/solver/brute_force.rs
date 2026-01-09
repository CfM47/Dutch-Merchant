use crate::evaluator::path_evaluator::PathEvaluator;
use crate::evaluator::LpProfitCalculator;
use crate::model::{instance::Instance, solution::Solution};
use crate::solver::solver::Solver;

pub struct BruteForceSolver {
    evaluator: LpProfitCalculator,
}

impl BruteForceSolver {
    pub fn new() -> Self {
        Self {
            evaluator: LpProfitCalculator::new(),
        }
    }

    fn solve_recursive(
        &self,
        instance: &Instance,
        current_path: &mut Vec<usize>,
        current_time: f64,
        best_profit: &mut f64,
        best_solution: &mut Option<Solution>,
    ) {
        let current_port = *current_path.last().unwrap();

        for next_port in 0..instance.n_ports {
            let travel_time = instance.travel_time[current_port][next_port];
            
            if current_time + travel_time <= instance.time_limit {
                if next_port == instance.start_port {
                    // Try to return to start port to complete the cycle
                    current_path.push(next_port);
                    let (profit, transactions) = self.evaluator.calculate_best_profit(instance, current_path);
                    // println!("Path: {:?}, Profit: {}", current_path, profit);   
                    if profit > *best_profit {
                        *best_profit = profit;
                        *best_solution = Some(Solution {
                            route: current_path.clone(),
                            transactions,
                        });
                    }
                    current_path.pop();
                } else if !current_path.contains(&next_port) {
                    // Visit a new port
                    current_path.push(next_port);
                    self.solve_recursive(
                        instance,
                        current_path,
                        current_time + travel_time,
                        best_profit,
                        best_solution,
                    );
                    current_path.pop();
                }
            }
        }
    }
}

impl Solver for BruteForceSolver {
    fn name(&self) -> &'static str {
        "BruteForceSolver"
    }

    fn solve(&self, instance: &Instance) -> Option<Solution> {
        let mut best_profit = f64::NEG_INFINITY;
        let mut best_solution = None;
        let mut current_path = vec![instance.start_port];

        self.solve_recursive(
            instance,
            &mut current_path,
            0.0,
            &mut best_profit,
            &mut best_solution,
        );

        best_solution
    }
}
#[pyo3::pyfunction]
pub fn brute_force_solve(instance: &Instance) -> Option<Solution> {
    let solver = BruteForceSolver::new();
    solver.solve(instance)
}
