mod evaluator;
mod model;
mod solver;

use crate::model::instance::Instance;
use crate::solver::{BruteForceSolver, Solver};

fn main() {
    println!("Dutch Merchant - Brute Force Solver");

    let instance = Instance {
        n_ports: 5,
        n_goods: 2,
        travel_time: vec![
            vec![0.0, 10.0, 15.0, 20.0, 25.0],
            vec![10.0, 0.0, 12.0, 18.0, 22.0],
            vec![15.0, 12.0, 0.0, 10.0, 15.0],
            vec![20.0, 18.0, 10.0, 0.0, 10.0],
            vec![25.0, 22.0, 15.0, 10.0, 0.0],
        ],
        travel_cost: 0.1,
        weight: vec![1.0, 2.0],
        buy_price: vec![
            vec![100.0, 200.0],
            vec![50.0, 180.0],
            vec![150.0, 250.0],
            vec![80.0, 210.0],
            vec![120.0, 230.0],
        ],
        sell_price: vec![
            vec![90.0, 190.0],
            vec![45.0, 170.0],
            vec![140.0, 240.0],
            vec![75.0, 200.0],
            vec![110.0, 220.0],
        ],
        buy_cap: vec![
            vec![10.0, 5.0],
            vec![10.0, 5.0],
            vec![10.0, 5.0],
            vec![10.0, 5.0],
            vec![10.0, 5.0],
        ],
        sell_cap: vec![
            vec![10.0, 5.0],
            vec![10.0, 5.0],
            vec![10.0, 5.0],
            vec![10.0, 5.0],
            vec![10.0, 5.0],
        ],
        visit_cost: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        start_port: 0,
        capacity: 100.0,
        time_limit: 100.0,
        initial_capital: 1000.0,
    };

    let solver = BruteForceSolver::new();
    println!("Running solver: {}", solver.name());

    if let Some(solution) = solver.solve(&instance) {
        println!("Best route: {:?}", solution.route);
        println!("Transactions: {:?}", solution.transactions);
        
        // Evaluate the solution to show final profit
        let evaluator = crate::evaluator::LpProfitCalculator::new();
        let (profit, _) = crate::evaluator::path_evaluator::PathEvaluator::calculate_best_profit(
            &evaluator, 
            &instance, 
            &solution.route
        );
        println!("Total Profit: {}", profit);
    } else {
        println!("No valid solution found within time limit.");
    }
}
