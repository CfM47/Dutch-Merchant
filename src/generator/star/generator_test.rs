use std::{iter::zip, path};

use crate::{
    Instance,
    evaluator::{IntervalEvaluator, path_evaluator::PathEvaluator},
    generator::star::{PathRequest, generate_instance},
};

fn assert_close(a: f64, b: f64, message: &'static str) {
    if f64::abs(a - b) > 1E-8 {
        dbg!(a, b);
        panic!("assertion failed: {}", message);
    }
}

fn test_path_with_interval_evaluator(
    instance: &Instance,
    expected_path: &[usize],
    travel_expense: f64,
    earnings: f64,
) {
    dbg!(&instance);
    dbg!(&expected_path);

    let mut actual_travel_expense = 0_f64;
    for (&i, &j) in zip(
        expected_path[0..expected_path.len() - 1].iter(),
        expected_path[1..expected_path.len()].iter(),
    ) {
        actual_travel_expense +=
            dbg!(instance.travel_time[i][j] * instance.travel_cost + instance.visit_cost[j]);
    }

    assert_eq!(actual_travel_expense, travel_expense);

    let interval_evaluator = IntervalEvaluator::new();
    let path_solution = interval_evaluator.calculate_best_profit(instance, expected_path);

    dbg!(&instance.buy_price);
    dbg!(&instance.sell_price);
    dbg!(&path_solution);

    assert_close(
        path_solution.0,
        earnings - travel_expense,
        "wrong final balance",
    );
}

#[test]
fn simple_instance() {
    let travel_expense = 10_f64;
    let earnings = 1_f64;

    let request = PathRequest {
        path: vec![1, 2, 3],
        extra_ports: 0,
        earnings,
        travel_expense,
        offer_generation_strategy:
            crate::generator::star::OfferGenerationStrategy::SingleProductBuyFirstSellLast,
        travel_time_generation_strategy: crate::generator::star::TravelTimeGenerationStrategy::Even,
    };
    let instance = generate_instance(vec![request], Some(42)).unwrap();

    assert_eq!(
        (instance.travel_time.len(), instance.travel_time[0].len()),
        (4, 4)
    );

    assert_eq!(instance.n_ports, 4);

    let expected_path: Vec<usize> = vec![0, 1, 2, 3, 0];
    test_path_with_interval_evaluator(&instance, &expected_path, travel_expense, earnings);
}

#[test]
fn simple_instance_permuted() {
    let travel_expense = 10_f64;
    let earnings = 1_f64;

    let request = PathRequest {
        path: vec![2, 1, 4],
        extra_ports: 0,
        earnings,
        travel_expense,
        offer_generation_strategy:
            crate::generator::star::OfferGenerationStrategy::SingleProductBuyFirstSellLast,
        travel_time_generation_strategy: crate::generator::star::TravelTimeGenerationStrategy::Even,
    };
    let instance = generate_instance(vec![request], Some(42)).unwrap();

    assert_eq!(
        (instance.travel_time.len(), instance.travel_time[0].len()),
        (5, 5)
    );

    assert_eq!(instance.n_ports, 5);

    let expected_path: Vec<usize> = vec![0, 2, 1, 4, 0];
    test_path_with_interval_evaluator(&instance, &expected_path, travel_expense, earnings);
}

#[test]
fn simple_two_path() {
    let travel_expense_1 = 10_f64;
    let earnings_1 = 1_f64;
    let request_1 = PathRequest {
        path: vec![1, 2, 3],
        extra_ports: 0,
        earnings: earnings_1,
        travel_expense: travel_expense_1,
        offer_generation_strategy:
            crate::generator::star::OfferGenerationStrategy::SingleProductBuyFirstSellLast,
        travel_time_generation_strategy: crate::generator::star::TravelTimeGenerationStrategy::Even,
    };

    let travel_expense_2 = 20_f64;
    let earnings_2 = 30_f64;
    let request_2 = PathRequest {
        path: vec![2, 1, 3],
        extra_ports: 0,
        earnings: earnings_2,
        travel_expense: travel_expense_2,
        offer_generation_strategy:
            crate::generator::star::OfferGenerationStrategy::SingleProductBuyFirstSellLast,
        travel_time_generation_strategy: crate::generator::star::TravelTimeGenerationStrategy::Even,
    };

    let instance = generate_instance(vec![request_1, request_2], Some(42)).unwrap();

    assert_eq!(
        (instance.travel_time.len(), instance.travel_time[0].len()),
        (7, 7)
    );

    assert_eq!(instance.n_ports, 7);

    let expected_path: Vec<usize> = vec![0, 1, 2, 3, 0];
    test_path_with_interval_evaluator(&instance, &expected_path, travel_expense_1, earnings_1);

    let expected_path: Vec<usize> = [0, 2, 1, 3, 0]
        .iter()
        .map(|&i| if i == 0 { i } else { i + 3 })
        .collect();

    test_path_with_interval_evaluator(&instance, &expected_path, travel_expense_2, earnings_2);
}

#[test]
fn simple_extra_ports() {
    let travel_expense = 10_f64;
    let earnings = 1_f64;

    let request = PathRequest {
        path: vec![1, 2, 3],
        extra_ports: 1,
        earnings,
        travel_expense,
        offer_generation_strategy:
            crate::generator::star::OfferGenerationStrategy::SingleProductBuyFirstSellLast,
        travel_time_generation_strategy: crate::generator::star::TravelTimeGenerationStrategy::Even,
    };
    let instance = generate_instance(vec![request], Some(42)).unwrap();
    dbg!(&instance);

    assert_eq!(
        (instance.travel_time.len(), instance.travel_time[0].len()),
        (5, 5)
    );

    assert_eq!(instance.n_ports, 5);

    let expected_path: Vec<usize> = vec![0, 1, 2, 3, 0];
    test_path_with_interval_evaluator(&instance, &expected_path, travel_expense, earnings);
}

#[test]
fn n_paths_1() {
    let travel_expenses = [10_f64, 20_f64, 300_f64];
    let earnings = [0_f64, 200_f64, 500_f64];
    let paths = [vec![2, 4, 1], vec![1, 2], vec![3, 2, 1]];
    let extra_ports = [1, 0, 10];

    let mut requests = Vec::new();
    for i in 0..3 {
        requests.push(PathRequest {
            path: paths[i].clone(),
            extra_ports: extra_ports[i],
            earnings: earnings[i],
            offer_generation_strategy:
                crate::generator::star::OfferGenerationStrategy::SingleProductBuyFirstSellLast,
            travel_expense: travel_expenses[i],
            travel_time_generation_strategy:
                crate::generator::star::TravelTimeGenerationStrategy::Even,
        })
    }

    let instance = generate_instance(requests.to_vec(), Some(42)).unwrap();

    let mut max_port_id = 0;
    for request in requests {
        let expected_path: Vec<usize> = vec![0]
            .into_iter()
            .chain(request.path.iter().map(|p| p + max_port_id))
            .chain(vec![0])
            .collect();

        test_path_with_interval_evaluator(
            &instance,
            &expected_path,
            request.travel_expense,
            request.earnings,
        );

        max_port_id += request.path.iter().max().unwrap() + request.extra_ports;
        dbg!(max_port_id);
    }
}

#[test]
fn n_paths_2() {
    let travel_expenses = [10_f64, 20_f64, 10_f64, 3_f64, 40_f64, 0.999999];
    let earnings = [0_f64, 200_f64, 1_f64, 500_f64, 2_f64, 0_f64];
    let paths = [
        vec![1, 2, 3],
        vec![2, 4, 1],
        vec![10, 2, 3],
        vec![4, 3, 2, 5, 6],
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1, 2],
    ];
    let extra_ports = [1, 0, 0, 0, 0, 10];

    let mut requests = Vec::new();
    for i in 0..paths.len() {
        requests.push(PathRequest {
            path: paths[i].clone(),
            extra_ports: extra_ports[i],
            earnings: earnings[i],
            offer_generation_strategy:
                crate::generator::star::OfferGenerationStrategy::SingleProductBuyFirstSellLast,
            travel_expense: travel_expenses[i],
            travel_time_generation_strategy:
                crate::generator::star::TravelTimeGenerationStrategy::Even,
        })
    }

    let instance = generate_instance(requests.to_vec(), Some(42)).unwrap();

    let mut max_port_id = 0;
    for request in requests {
        let expected_path: Vec<usize> = vec![0]
            .into_iter()
            .chain(request.path.iter().map(|p| p + max_port_id))
            .chain(vec![0])
            .collect();

        max_port_id += request.path.iter().max().unwrap() + request.extra_ports;
        dbg!(max_port_id);

        test_path_with_interval_evaluator(
            &instance,
            &expected_path,
            request.travel_expense,
            request.earnings,
        );
    }
}


#[test]
fn n_paths_3() {
    let travel_expenses = [
        // 10_f64, 
        // 20_f64, 
        10_f64, 
        3_f64, 
        // 40_f64, 
        // 0.999999
    ];
    let earnings = [
        // 0_f64, 
        // 200_f64, 
        1_f64, 
        500_f64, 
        // 2_f64, 
        // 0_f64
    ];
    let paths = [
        // vec![1, 2, 3],
        // vec![2, 4, 1],
        vec![2, 3],
        vec![4, 3, 2, 5, 6],
        // vec![1, 2, 3, 4, 5, 6, 7, 8],
        // vec![1, 2],
    ];
    let extra_ports = [
        // 1, 
        // 0, 
        0, 
        0, 
        // 0, 
        // 10
    ];

    let mut requests = Vec::new();
    for i in 0..paths.len() {
        requests.push(PathRequest {
            path: paths[i].clone(),
            extra_ports: extra_ports[i],
            earnings: earnings[i],
            offer_generation_strategy:
                crate::generator::star::OfferGenerationStrategy::SingleProductBuyFirstSellLast,
            travel_expense: travel_expenses[i],
            travel_time_generation_strategy:
                crate::generator::star::TravelTimeGenerationStrategy::Even,
        })
    }

    let instance = generate_instance(requests.to_vec(), Some(42)).unwrap();

    let mut max_port_id = 0;
    for request in requests {
        let expected_path: Vec<usize> = vec![0]
            .into_iter()
            .chain(request.path.iter().map(|p| p + max_port_id))
            .chain(vec![0])
            .collect();

        max_port_id += request.path.iter().max().unwrap() + request.extra_ports;
        dbg!(max_port_id);

        test_path_with_interval_evaluator(
            &instance,
            &expected_path,
            request.travel_expense,
            request.earnings,
        );
    }
}
