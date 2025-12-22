use std::{collections::HashMap, iter::zip};

use crate::evaluator::path_evaluator::PathEvaluator;

use super::Interval;

#[derive(Clone, Debug)]
struct PartialSolution<'a>(f64, Vec<&'a Interval>);

pub struct IntervalEvaluator {}

impl Default for IntervalEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl IntervalEvaluator {
    pub fn new() -> Self {
        Self {}
    }

    fn construct_intervals(
        instance: &crate::model::instance::Instance,
        nodes: &[crate::model::instance::PortId],
    ) -> HashMap<usize, Vec<Interval>> {
        let mut answ = HashMap::new();

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                for p in 0..instance.n_goods {
                    let result = match answ.entry(j) {
                        std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                            occupied_entry.into_mut()
                        }
                        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(Vec::new())
                        }
                    };

                    if instance.get_buy_price(nodes[i], p) >= instance.get_sell_price(nodes[j], p) {
                        continue;
                    }

                    result.push(Interval::new(p, i, j));
                }
            }
        }

        answ
    }

    fn fill_dp<'a>(
        instance: &crate::model::instance::Instance,
        nodes: &[crate::model::instance::PortId],
        intervals: &'a HashMap<usize, Vec<Interval>>,
    ) -> Vec<PartialSolution<'a>> {
        let mut dp: Vec<PartialSolution> = vec![];

        for i in 0..nodes.len() {
            let previous_option = if i > 0 { Some(&dp[i - 1]) } else { None };
            let mut best_option: Option<PartialSolution> = None;

            if let Some(interval_list) = intervals.get(&i) {
                let mut best_interval_choice: Option<(f64, &Interval)> = None;

                for interval in interval_list {
                    let buy_price =
                        instance.get_buy_price(nodes[interval.buy_port_index], interval.product);
                    let sell_price =
                        instance.get_sell_price(nodes[interval.sell_port_index], interval.product);

                    let profit = (sell_price - buy_price) * instance.capacity
                        / instance.get_weight(interval.product);

                    let previous_profit = if interval.buy_port_index > 0 {
                        dp[interval.buy_port_index].0
                    } else {
                        0.0
                    };

                    let take_profit = profit + previous_profit;
                    let not_take_profit = previous_option.map(|v| v.0).unwrap_or(0.0);

                    if not_take_profit > take_profit {
                        continue;
                    }

                    if let Some(choice) = best_interval_choice
                        && choice.0 > take_profit
                    {
                        continue;
                    }

                    best_interval_choice = Some((take_profit, interval));
                }

                if let Some((profit, chosen_interval)) = best_interval_choice {
                    let mut new_intervals = if chosen_interval.buy_port_index > 0 {
                        dp[chosen_interval.buy_port_index].1.clone()
                    } else {
                        Vec::new()
                    };

                    new_intervals.push(chosen_interval);
                    best_option = Some(PartialSolution(profit, new_intervals));
                }
            }

            dp.push(match best_option {
                Some(option) => option.clone(),
                None => previous_option
                    .cloned()
                    .unwrap_or(PartialSolution(0.0, Vec::new())),
            });
        }

        dp
    }

    fn map_answer(
        instance: &crate::model::instance::Instance,
        solution: &PartialSolution,
        nodes: &[crate::model::instance::PortId],
    ) -> (f64, Vec<Vec<f64>>) {
        let mut answ: Vec<Vec<f64>> = (0..nodes.len())
            .map(|_| vec![0.0; instance.n_goods])
            .collect();

        for interval in solution.1.iter() {
            answ[interval.buy_port_index][interval.product] =
                instance.capacity / instance.get_weight(interval.product);
            answ[interval.sell_port_index][interval.product] =
                -(instance.capacity / instance.get_weight(interval.product));
        }

        let expense = Self::calculate_expense(instance, nodes);

        (solution.0 - expense + instance.initial_capital, answ)
    }

    fn calculate_expense(
        instance: &crate::model::instance::Instance,
        nodes: &[crate::model::instance::PortId],
    ) -> f64 {
        let mut answ = 0.0;

        for (&i, &j) in zip(
            nodes[0..(nodes.len() - 1)].iter(),
            nodes[1..nodes.len()].iter(),
        ) {
            answ += instance.travel_time[i][j] * instance.travel_cost + instance.visit_cost[j];
        }

        answ
    }
}

impl PathEvaluator for IntervalEvaluator {
    fn name(&self) -> &'static str {
        "IntervalEvaluator"
    }

    fn calculate_best_profit(
        &self,
        instance: &crate::model::instance::Instance,
        nodes: &[crate::model::instance::PortId],
    ) -> (f64, Vec<Vec<f64>>) {
        if nodes.is_empty() {
            return (0.0, vec![]);
        }

        let intervals = Self::construct_intervals(instance, nodes);

        let dp = Self::fill_dp(instance, nodes, &intervals);

        Self::map_answer(instance, dp.last().unwrap(), nodes)
    }
}
