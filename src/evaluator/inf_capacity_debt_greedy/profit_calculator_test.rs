use crate::evaluator::path_evaluator::PathEvaluator;
use crate::{
    evaluator::inf_capacity_debt_greedy::profit_calculator::InfiniteCapacityDebtEvaluator,
    model::instance::{Instance, PortId},
};

#[test]
fn test_calculate_best_profit_simple_case() {
    // 2 ports, 1 good
    let instance = Instance {
        n_ports: 2,
        n_goods: 1,
        travel_time: vec![vec![0.0, 1.0], vec![1.0, 0.0]],
        travel_cost: 0.0,
        weight: vec![1.0],
        buy_price: vec![vec![2.0], vec![0.0]], // Only port 0 sells
        sell_price: vec![vec![0.0], vec![5.0]], // Only port 1 buys
        buy_cap: vec![vec![10.0], vec![0.0]],
        sell_cap: vec![vec![0.0], vec![10.0]],
        visit_cost: vec![0.0, 0.0],
        start_port: 0,
        capacity: 10.0,
        time_limit: 100.0,
        initial_capital: 20.0,
    };
    // Route: port 0 -> port 1
    let route: Vec<PortId> = vec![0, 1];
    let evaluator = InfiniteCapacityDebtEvaluator::new();
    let (profit, _) = evaluator.calculate_best_profit(&instance, &route);
    // Buy 10 at port 0 for 2 each, sell 10 at port 1 for 5 each
    // Initial: 20, buy: -20, sell: +50, final: 50
    assert!(
        (profit - 50.0).abs() < 1e-6,
        "Expected profit 50.0, got {}",
        profit
    );
}

#[test]
fn test_calculate_best_profit_liquidity_and_multi_stop() {
    // 4 ports, 2 goods
    // Good 0: Cheap (Weight 1.0, Buy 10, Sell 20)
    // Good 1: Luxury (Weight 2.0, Buy 200, Sell 1000)
    let instance = Instance {
        n_ports: 4,
        n_goods: 2,
        travel_time: vec![
            vec![0.0, 1.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0, 1.0],
            vec![1.0, 1.0, 0.0, 1.0],
            vec![1.0, 1.0, 1.0, 0.0],
        ],
        travel_cost: 0.0,
        weight: vec![1.0, 2.0],
        buy_price: vec![
            vec![10.0, 200.0], // Port 0: Luxury too expensive for initial 100
            vec![0.0, 0.0],
            vec![0.0, 200.0], // Port 2: Buy Luxury here after making money
            vec![0.0, 0.0],
        ],
        sell_price: vec![
            vec![0.0, 0.0],
            vec![20.0, 0.0], // Port 1: Sell Cheap good to get cash
            vec![0.0, 0.0],
            vec![0.0, 1000.0], // Port 3: Big payout
        ],
        buy_cap: vec![
            vec![10.0, 5.0],
            vec![0.0, 0.0],
            vec![0.0, 5.0],
            vec![0.0, 0.0],
        ],
        sell_cap: vec![
            vec![0.0, 0.0],
            vec![10.0, 0.0],
            vec![0.0, 0.0],
            vec![0.0, 5.0],
        ],
        visit_cost: vec![0.0, 0.0, 0.0, 0.0],
        start_port: 0,
        capacity: 10.0,
        time_limit: 100.0,
        initial_capital: 105.0, // Enough for 10 units of Good 0 + visit cost
    };

    // Route: 0 -> 1 -> 2 -> 3
    let route: Vec<PortId> = vec![0, 1, 2, 3];
    let evaluator = InfiniteCapacityDebtEvaluator::new();
    let (profit, _) = evaluator.calculate_best_profit(&instance, &route);

    /* Step-by-Step Logic:
        1. Port 0: Spend 100 to buy 10 units of Good 0, spend 1000 to buy 5 units of Good 1. Capital left: -995.
        2. Port 1: Sell 10 units of Good 0 for 200. New Capital: -795.
        3. Port 2: does nothing. Capital: -795.
        4. Port 3: Sell 5 units of Good 1 for 5000. New Capital: 4205.
    */

    assert!(
        (profit - 4205.0).abs() < 1e-6,
        "The merchant should at least make more than initial capital"
    );
}

#[test]
fn test_pair_greatest_sell_to_greatest_buy() {
    // 3 ports, 1 good
    let instance = Instance {
        n_ports: 3,
        n_goods: 1,
        travel_time: vec![
            vec![0.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0],
            vec![1.0, 1.0, 0.0],
        ],
        travel_cost: 0.0,
        weight: vec![1.0],
        buy_price: vec![
            vec![2.0],
            vec![0.0],
            vec![0.0],
        ],
        sell_price: vec![
            vec![0.0],
            vec![5.0],
            vec![10.0],
        ],
        buy_cap: vec![
            vec![1.0],
            vec![0.0],
            vec![0.0],
        ],
        sell_cap: vec![
            vec![0.0],
            vec![1.0],
            vec![1.0],
        ],
        visit_cost: vec![0.0, 0.0, 0.0],
        start_port: 0,
        capacity: 1000000.0,
        time_limit: 100000.0,
        initial_capital: 0.0, // Enough for 10 units of Good 0 + visit cost
    };

    // Route: 0 -> 1 -> 2
    let route: Vec<PortId> = vec![0, 1, 2];
    let evaluator = InfiniteCapacityDebtEvaluator::new();
    let (profit, _) = evaluator.calculate_best_profit(&instance, &route);

    assert!(
        (profit - 8.0).abs() < 1e-6,
        "Invalid profit: Expected 8, got {}",
        profit
    );
}