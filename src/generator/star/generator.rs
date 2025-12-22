use std::iter::zip;

use crate::Instance;
use super::{
    GeneratedOffer, GeneratedPathInstance, PathRequest,
    matrix::{extend_matrix, gapped_array, prepend_column_and_row, prepend_row},
};
use rand::SeedableRng;
use rand::prelude::*;
use rand::rngs::StdRng;

pub fn generate_instance(
    path_requests: Vec<PathRequest>,
    seed: Option<u64>,
) -> Result<Instance, &'static str> {
    let mut rng = match seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_os_rng(),
    };

    let g = Generator::build(path_requests, rng.random(), 0_f64)?;
    Ok(g.generate(&mut rng))
}

struct Generator {
    initial_capital: f64,
    infinite_time: f64,
    infinite_price: f64,
    travel_cost: f64,
    requests: Vec<PathRequest>,
}

impl Generator {
    fn build(
        requests: Vec<PathRequest>,
        travel_cost: f64,
        initial_capital: f64,
    ) -> Result<Self, &'static str> {
        let max_earnings = requests
            .iter()
            .map(|r| r.earnings)
            .reduce(f64::max)
            .unwrap_or(0.0);

        let max_travel_expense = requests
            .iter()
            .map(|r| r.travel_expense)
            .reduce(f64::max)
            .unwrap_or(0.0);

        let answ = Self {
            initial_capital,
            infinite_time: (max_travel_expense + 1_f64) * 2_f64 / travel_cost,
            infinite_price: (initial_capital + max_earnings + 1_f64) * 2_f64,
            travel_cost,
            requests,
        };

        if answ.requests.is_empty() {
            return Err("At least one path request must be provided");
        }
        // given that port 0 will be zero-cost linked to all other ports, we
        // need at least two ports to have travel expenses
        if answ.requests.iter().any(|r| r.path.len() < 2) {
            return Err("Each path request must have at least two ports in its path");
        }
        if answ.requests.iter().any(|r| r.path.contains(&0)) {
            return Err("Port IDs in path requests must be greater than 0");
        }

        Ok(answ)
    }

    fn generate(self, rng: &mut StdRng) -> Instance {
        let capacity = rng.random();

        let mut path_generations: Vec<GeneratedPathInstance> = Vec::new();

        for request in self.requests.iter() {
            path_generations.push(self.generate_sub_instance(
                request,
                self.travel_cost,
                capacity,
                rng,
            ));
        }

        let travel_time = path_generations
            .iter()
            .map(|g| g.travel_distance.clone())
            .reduce(|acc, e| extend_matrix(acc, e, &self.infinite_time))
            .expect("There must be at least one path generation");
        let travel_time = prepend_column_and_row(travel_time, &0_f64);

        // make zero-cost, zero-profit link from port 0 to all other ports
        let buy_price = path_generations
            .iter()
            .map(|g| dbg!(g.offer.buy_price.clone()))
            .reduce(|acc, e| extend_matrix(acc, e, &self.infinite_price))
            .unwrap();
        let buy_price = prepend_row(buy_price, &self.infinite_price);

        let sell_price = path_generations
            .iter()
            .map(|g| g.offer.sell_price.clone())
            .reduce(|acc, e| extend_matrix(acc, e, &0_f64))
            .unwrap();
        let sell_price = prepend_row(sell_price, &0_f64);

        let buy_cap = path_generations
            .iter()
            .map(|g| g.offer.buy_cap.clone())
            .reduce(|acc, e| extend_matrix(acc, e, &0_f64))
            .unwrap();
        let buy_cap = prepend_row(buy_cap, &0_f64);

        let sell_cap = path_generations
            .iter()
            .map(|g| g.offer.sell_cap.clone())
            .reduce(|acc, e| extend_matrix(acc, e, &0_f64))
            .unwrap();
        let sell_cap = prepend_row(sell_cap, &0_f64);

        let weight: Vec<f64> = path_generations
            .iter()
            .flat_map(|g| g.offer.product_weight.clone())
            .collect();

        let n_ports: usize = buy_price.len();
        let n_goods: usize = weight.len();

        Instance {
            n_ports,
            n_goods,
            travel_cost: self.travel_cost,
            initial_capital: self.initial_capital,
            capacity,
            //  FIXME: later
            visit_cost: vec![0.0; n_ports],
            weight,
            travel_time,
            buy_price,
            sell_price,
            buy_cap,
            sell_cap,

            start_port: 0,
            time_limit: 0.0,
        }
    }

    fn generate_sub_instance(
        &self,
        path_request: &PathRequest,
        travel_cost: f64,
        capacity: f64,
        rng: &mut StdRng,
    ) -> GeneratedPathInstance {
        let offer = match path_request.offer_generation_strategy {
            super::OfferGenerationStrategy::SingleProductBuyFirstSellLast => {
                        self.offer_buy_single_product_first_sell_last(path_request, capacity, rng)
                    }
        };

        let travel_distance = match path_request.travel_time_generation_strategy {
            super::TravelTimeGenerationStrategy::Even => {
                self.even_travel_costs(path_request, travel_cost)
            }
        };

        GeneratedPathInstance {
            offer,
            travel_distance,
        }
    }

    fn offer_buy_single_product_first_sell_last(
        &self,
        path_request: &PathRequest,
        capacity: f64,
        rng: &mut StdRng,
    ) -> GeneratedOffer {
        let n_ports = Self::calculate_ports(path_request);
        let path_len = path_request.path.len();
        let normalized_path = path_request
            .path
            .iter()
            .map(|p| p - 1)
            .collect::<Vec<usize>>();

        // less than infinite price
        let first_buy_price = rng.random::<f64>() * self.infinite_price;
        let last_sell_price =
            first_buy_price + rng.random::<f64>() * (self.infinite_price - first_buy_price);

        // (s - b) * a = earnings
        let product_amount = path_request.earnings / (last_sell_price - first_buy_price);

        // w * a == capacity
        let product_weight = capacity / product_amount;

        let buy_price = vec![vec![first_buy_price]]
            .into_iter()
            .chain(vec![vec![self.infinite_price]; path_len - 1])
            .collect();
        let buy_price = gapped_array(
            buy_price,
            &normalized_path,
            &vec![self.infinite_price],
            n_ports,
        );

        let buy_cap = vec![vec![capacity]]
            .into_iter()
            .chain(vec![vec![0 as f64]; path_len - 1])
            .collect();
        let buy_cap = gapped_array(buy_cap, &normalized_path, &vec![0.0], n_ports);

        let sell_price = vec![vec![0 as f64]; path_len - 1]
            .into_iter()
            .chain(vec![vec![last_sell_price]])
            .collect();
        let sell_price = gapped_array(sell_price, &normalized_path, &vec![0.0], n_ports);

        let sell_cap = vec![vec![0 as f64]; path_len - 1]
            .into_iter()
            .chain(vec![vec![capacity]])
            .collect();
        let sell_cap = gapped_array(sell_cap, &normalized_path, &vec![0.0], n_ports);

        GeneratedOffer {
            buy_price,
            buy_cap,
            sell_price,
            sell_cap,

            product_weight: vec![product_weight],
        }
    }

    fn even_travel_costs(&self, path_request: &PathRequest, travel_cost: f64) -> Vec<Vec<f64>> {
        let n_ports = Self::calculate_ports(path_request);
        let time_between_ports =
            path_request.travel_expense / travel_cost / (path_request.path.len() - 1) as f64;

        let mut travel_time = vec![vec![self.infinite_time; n_ports]; n_ports];

        for (&i, &j) in zip(
            path_request.path[0..path_request.path.len() - 1].iter(),
            path_request.path.iter().skip(1),
        ) {
            travel_time[i - 1][j - 1] = time_between_ports
        }
        for i in 0..travel_time.len() {
            travel_time[i][i] = 0_f64;
        }

        travel_time
    }

    fn calculate_ports(path_request: &PathRequest) -> usize {
        path_request.path.iter().max().unwrap() + path_request.extra_ports
    }
}
