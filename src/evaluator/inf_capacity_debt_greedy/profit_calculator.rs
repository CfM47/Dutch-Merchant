use std::{collections::BinaryHeap, vec};

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
    buy_price: f64,
    port_idx: usize,
    buy_cap: f64,
}

impl HeapItem {
    fn new(buy_price: f64, port_idx: usize, buy_cap: f64) -> Self {
        HeapItem {
            buy_price,
            port_idx,
            buy_cap,
        }
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.buy_price.partial_cmp(&self.buy_price).unwrap()
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

        let mut profit = instance.initial_capital;
        // q[j][m]
        let mut decisions: Vec<Vec<f64>> = vec![vec![0.0; n_goods]; n_ports];

        for m in 0..n_goods {
            let mut heap = BinaryHeap::new();

            for j in 0..n_ports {
                let buy_price = instance.buy_price[nodes[j]][m];
                let buy_cap = instance.buy_cap[nodes[j]][m];
                let sell_price = instance.sell_price[nodes[j]][m];
                let mut sell_cap = instance.sell_cap[nodes[j]][m];

                // if can be bought, add buys to the heap
                if buy_cap > 0.0 {
                    heap.push(HeapItem::new(buy_price, j, buy_cap));
                }

                while sell_cap > 0.0 && !heap.is_empty() {
                    // pop the item with the highest buy price
                    let mut item = heap.pop().unwrap();

                    // calculate profit of buying and selling in greedy fashion
                    let take = sell_cap.min(item.buy_cap);
                    profit += take * (sell_price - item.buy_price);

                    // update quantities
                    item.buy_cap -= take;
                    sell_cap -= take;
                    if item.buy_cap > 0.0 {
                        heap.push(item);
                    }

                    // update decisions
                    decisions[item.port_idx][m] += take;
                    decisions[j][m] -= take;
                }
            }
        }

        for j in 1..nodes.len() {
            let travel_time = instance.travel_time[nodes[j - 1]][nodes[j]];
            let travel_cost = instance.travel_cost;
            let visit_cost = instance.visit_cost[nodes[j]];
            profit -= travel_time * travel_cost + visit_cost;
        }

        (profit, decisions)
    }
}
