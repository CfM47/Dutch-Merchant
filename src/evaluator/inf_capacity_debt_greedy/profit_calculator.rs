use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    vec,
};

use crate::{
    evaluator::path_evaluator::PathEvaluator,
    model::instance::{Instance, PortId},
};

/// # Description
///
/// Infinite capacity debt greedy profit calculator
/// Calculates the best profit given an instance and a given sequence of nodes (ports).
/// It assumes that the boat capacity is infinite and that the capital of the boat can be negative, which means it can be used as a debt.
pub struct InfiniteCapacityDebtEvaluator;

impl InfiniteCapacityDebtEvaluator {
    pub fn new() -> Self {
        InfiniteCapacityDebtEvaluator
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
struct HeapItem {
    price: f64,
    idx: usize,
}

impl HeapItem {
    fn new(price: f64, idx: usize) -> Self {
        HeapItem {
            price: price,
            idx: idx,
        }
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.price.partial_cmp(&self.price).unwrap()
    }
}

impl Eq for HeapItem {}

impl PathEvaluator for InfiniteCapacityDebtEvaluator {
    fn name(&self) -> &'static str {
        return "InfiniteCapacityDebtEvaluator";
    }

    fn calculate_best_profit(&self, instance: &Instance, nodes: &[PortId]) -> (f64, Vec<Vec<f64>>) {
        let n_goods = instance.n_goods;
        let n_ports = nodes.len();

        let mut capital = instance.initial_capital;
        // q[j][m]
        // TODO: fix this into a vec of tuples
        let mut decisions: Vec<Vec<f64>> = vec![vec![0.0; n_goods]; n_ports];

        for m in 0..n_goods {
            // fix the problem for good m
            let buy_prices: Vec<f64> = nodes.iter().map(|x| instance.buy_price[*x][m]).collect();
            let sell_prices: Vec<f64> = nodes.iter().map(|x| instance.sell_price[*x][m]).collect();
            let mut buy_cap: Vec<f64> = nodes.iter().map(|x| instance.buy_cap[*x][m]).collect();
            let mut sell_cap: Vec<f64> = nodes.iter().map(|x| instance.sell_cap[*x][m]).collect();

            let mut sell_heap = BinaryHeap::new();

            for i in 0..nodes.len() {
                sell_heap.push(HeapItem::new(sell_prices[i], i));
            }

            while !sell_heap.is_empty() {
                let v_i = sell_heap.pop().unwrap();
                let sell_price = v_i.price;
                let i = v_i.idx;

                let mut buy_heap = BinaryHeap::new();

                for j in 0..i {
                    buy_heap.push(Reverse(HeapItem::new(buy_prices[j], j)));
                }

                while sell_cap[i] > 0.0 && !buy_heap.is_empty() {
                    let v_j = buy_heap.pop().unwrap().0;
                    let buy_price = v_j.price;
                    let j = v_j.idx;

                    if sell_price <= buy_price {
                        continue;
                    }
                    let x = sell_cap[i].min(buy_cap[j]);
                    // TODO:
                    // decisions[j][m].0 += x
                    // decisions[i][m].1 += x
                    sell_cap[i] -= x;
                    buy_cap[j] -= x;

                    capital += x * (sell_price - buy_price)
                }
            }
        }

        for j in 1..nodes.len() {
            let travel_time = instance.travel_time[nodes[j - 1]][nodes[j]];
            let travel_cost = instance.travel_cost;
            let visit_cost = instance.visit_cost[nodes[j]];
            capital -= travel_time * travel_cost + visit_cost;
        }

        (capital, decisions)
    }
}
