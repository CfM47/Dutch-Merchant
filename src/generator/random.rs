use crate::model::instance::{Instance, PortId};
use rand::Rng;

pub struct RandomConfig {
    pub n_ports: usize,
    pub n_goods: usize,
    pub travel_time_range: (f64, f64),
    pub price_range: (f64, f64),
    pub weight_range: (f64, f64),
    pub capacity_range: (f64, f64),
    pub time_limit_range: (f64, f64),
    pub initial_capital_range: (f64, f64),
    pub visit_cost_range: (f64, f64),
    pub travel_cost_range: (f64, f64),
}

impl Default for RandomConfig {
    fn default() -> Self {
        Self {
            n_ports: 5,
            n_goods: 3,
            travel_time_range: (1.0, 10.0),
            price_range: (10.0, 100.0),
            weight_range: (1.0, 5.0),
            capacity_range: (50.0, 200.0),
            time_limit_range: (50.0, 200.0),
            initial_capital_range: (100.0, 500.0),
            visit_cost_range: (5.0, 20.0),
            travel_cost_range: (0.1, 1.0),
        }
    }
}

pub fn generate_random_instance(config: &RandomConfig) -> Instance {
    let mut rng = rand::rng();

    let mut travel_time = vec![vec![0.0; config.n_ports]; config.n_ports];
    for i in 0..config.n_ports {
        for j in 0..config.n_ports {
            if i != j {
                travel_time[i][j] = rng.random_range(config.travel_time_range.0..config.travel_time_range.1);
            }
        }
    }

    let weight = (0..config.n_goods)
        .map(|_| rng.random_range(config.weight_range.0..config.weight_range.1))
        .collect();

    let mut buy_price = vec![vec![0.0; config.n_goods]; config.n_ports];
    let mut sell_price = vec![vec![0.0; config.n_goods]; config.n_ports];
    let mut buy_cap = vec![vec![0.0; config.n_goods]; config.n_ports];
    let mut sell_cap = vec![vec![0.0; config.n_goods]; config.n_ports];

    for p in 0..config.n_ports {
        for g in 0..config.n_goods {
            let base_price = rng.random_range(config.price_range.0..config.price_range.1);
            buy_price[p][g] = base_price * 1.1; // Buy is more expensive
            sell_price[p][g] = base_price * 0.9; // Sell is cheaper
            buy_cap[p][g] = rng.random_range(10.0..50.0);
            sell_cap[p][g] = rng.random_range(10.0..50.0);
        }
    }

    let visit_cost = (0..config.n_ports)
        .map(|_| rng.random_range(config.visit_cost_range.0..config.visit_cost_range.1))
        .collect();

    Instance {
        n_ports: config.n_ports,
        n_goods: config.n_goods,
        travel_time,
        travel_cost: rng.random_range(config.travel_cost_range.0..config.travel_cost_range.1),
        weight,
        buy_price,
        sell_price,
        buy_cap,
        sell_cap,
        visit_cost,
        start_port: rng.random_range(0..config.n_ports),
        capacity: rng.random_range(config.capacity_range.0..config.capacity_range.1),
        time_limit: rng.random_range(config.time_limit_range.0..config.time_limit_range.1),
        initial_capital: rng.random_range(config.initial_capital_range.0..config.initial_capital_range.1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generation() {
        let config = RandomConfig::default();
        let instance = generate_random_instance(&config);
        assert_eq!(instance.n_ports, config.n_ports);
        assert_eq!(instance.n_goods, config.n_goods);
    }
}
