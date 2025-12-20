use std::iter::zip;

use crate::{
    evaluator::{intervals::IntervalSolver, path_evaluator::PathEvaluator},
    model::instance::Instance,
};

#[test]
fn sanity_check() {
    let instance = test_instance(
        vec![1.0],
        vec![vec![1.0], vec![0.0]],
        vec![vec![0.0], vec![2.0]],
        2.0,
    );

    let solver = IntervalSolver::new();
    let solution = solver.calculate_best_profit(&instance, &[0, 1]);

    assert_eq!(solution.0, 2.0);
    assert_eq!(solution.1, [[2.0], [-2.0]])
}

#[test]
fn skips_bad_deal() {
    let instance = test_instance(
        vec![1.0, 1.0],
        vec![vec![1.0, 2.0], vec![2.0, 2.0], vec![0.0, 0.0]],
        vec![vec![0.0, 0.0], vec![0.0, 0.0], vec![3.0, 3.0]],
        2.0,
    );

    let solver = IntervalSolver::new();
    let solution = solver.calculate_best_profit(&instance, &[0, 1, 2]);

    assert_eq!(solution.0, 4.0);
    assert_eq!(solution.1, [[2.0, 0.0], [0.0, 0.0], [-2.0, 0.0]])
}

#[test]
fn shuffled_order() {
    let weight = vec![1.0, 1.0, 1.0];
    let buy_price = vec![
        vec![10.0, 1.0, 10.0],
        vec![10.0, 10.0, 10.0],
        vec![10.0, 10.0, 10.0],
    ];
    let sell_price = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 10.0, 0.0],
        vec![0.0, 0.0, 0.0],
    ];
    let permutation = [1, 0, 2];
    let perm_buy = permute(buy_price, &permutation);
    let perm_sell = permute(sell_price, &permutation);

    let instance = test_instance(weight, perm_buy, perm_sell, 2.0);

    let solver = IntervalSolver::new();
    let solution = solver.calculate_best_profit(&instance, &permutation);

    assert_eq!(solution.0, (10.0 - 1.0) * 2.0);
    assert_eq!(
        solution.1,
        permute(
            vec![[0.0, 2.0, 0.0], [0.0, -2.0, 0.0], [0.0, 0.0, 0.0]],
            &permutation
        )
    )
}

#[test]
fn shuffled_two_buys() {
    let weight = vec![1.0, 1.0, 1.0];
    let buy_price = vec![
        vec![10.0, 1.0, 10.0],
        vec![10.0, 10.0, 10.0],
        vec![10.0, 10.0, 2.0],
        vec![10.0, 10.0, 10.0],
        vec![10.0, 10.0, 10.0],
    ];
    let sell_price = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 10.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 10.0],
    ];
    let permutation = [1, 4, 3, 0, 2];
    let perm_buy = permute(buy_price, &permutation);
    let perm_sell = permute(sell_price, &permutation);

    let instance = test_instance(weight, perm_buy, perm_sell, 2.0);

    let solver = IntervalSolver::new();
    let solution = solver.calculate_best_profit(&instance, &permutation);

    assert_eq!(solution.0, (10.0 - 1.0) * 2.0 + (10.0 - 2.0) * 2.0);
    assert_eq!(
        solution.1,
        permute(
            vec![
                [0.0, 2.0, 0.0],
                [0.0, -2.0, 0.0],
                [0.0, 0.0, 2.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, -2.0],
            ],
            &permutation
        )
    )
}

fn test_instance(
    weight: Vec<f64>,
    buy_price: Vec<Vec<f64>>,
    sell_price: Vec<Vec<f64>>,
    capacity: f64,
) -> Instance {
    validate_instance(&weight, &buy_price, &sell_price, capacity);

    Instance {
        n_ports: buy_price.len(),
        n_goods: weight.len(),
        weight,
        buy_price,
        sell_price,
        capacity,

        // not important
        travel_time: Vec::new(),
        initial_capital: 0.0,
        time_limit: 0.0,
        buy_cap: Vec::new(),
        sell_cap: Vec::new(),
        visit_cost: Vec::new(),
        start_port: 0,
    }
}

fn validate_instance(
    weight: &[f64],
    buy_price: &[Vec<f64>],
    sell_price: &[Vec<f64>],
    capacity: f64,
) {
    assert!(!weight.is_empty());
    assert_eq!(buy_price.len(), sell_price.len());

    assert!(buy_price.iter().all(|prices| prices.len() == weight.len()));
    assert!(sell_price.iter().all(|prices| prices.len() == weight.len()));

    assert!(capacity >= 0.0);
}

fn permute<T>(v: Vec<T>, perm: &[usize]) -> Vec<T>
where
    T: Clone,
{
    let mut answ: Vec<T> = v.to_vec();

    for (i, e) in zip(perm, v) {
        answ[*i] = e;
    }

    answ
}
