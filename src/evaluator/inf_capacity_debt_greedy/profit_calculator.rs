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

impl PathEvaluator for InfiniteCapacityDebtEvaluator {
    fn name(&self) -> &'static str {
        return "InfiniteCapacityDebtEvaluator";
    }

    fn calculate_best_profit(
        &self,
        instance: &Instance,
        nodes: &[PortId],
    ) -> (f64, Vec<Vec<(f64, f64)>>) {
        let n_goods = instance.n_goods;
        let n_ports = nodes.len();

        let mut capital = instance.initial_capital;
        // q[j][m]
        let mut decisions: Vec<Vec<(f64, f64)>> = vec![vec![(0.0, 0.0); n_goods]; n_ports];

        for m in 0..n_goods {
            // fix the problem for good m
            let buy_prices: Vec<f64> = nodes.iter().map(|x| instance.buy_price[*x][m]).collect();
            let sell_prices: Vec<f64> = nodes.iter().map(|x| instance.sell_price[*x][m]).collect();
            let mut buy_cap: Vec<f64> = nodes.iter().map(|x| instance.buy_cap[*x][m]).collect();
            let mut sell_cap: Vec<f64> = nodes.iter().map(|x| instance.sell_cap[*x][m]).collect();

            let mut sell_order: Vec<(PortId, usize)> =
                nodes.iter().enumerate().map(|(i, x)| (*x, i)).collect();
            sell_order
                .sort_by(|(a, _), (b, _)| sell_prices[*b].partial_cmp(&sell_prices[*a]).unwrap());

            let mut buy_order: Vec<(PortId, usize)> =
                nodes.iter().enumerate().map(|(i, x)| (*x, i)).collect();
            buy_order
                .sort_by(|(a, _), (b, _)| buy_prices[*a].partial_cmp(&buy_prices[*b]).unwrap());

            for i in 0..sell_order.len() {
                let sell_price = sell_prices[sell_order[i].0];
                let sell_idx = sell_order[i].1;

                for j in 0..buy_order.len() {
                    if sell_cap[sell_idx] <= 0.0 {
                        break;
                    }
                    let buy_price = buy_prices[buy_order[j].0];
                    let buy_idx = buy_order[j].1;

                    if sell_price <= buy_price || buy_idx >= sell_idx || buy_cap[buy_idx] <= 0.0 {
                        continue;
                    }

                    let x = sell_cap[sell_idx].min(buy_cap[buy_idx]);

                    decisions[buy_idx][m].0 += x;
                    decisions[sell_idx][m].1 += x;
                    sell_cap[sell_idx] -= x;
                    buy_cap[buy_idx] -= x;

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
