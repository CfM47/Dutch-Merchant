use crate::model::instance::Instance;
use crate::generator::helpers::{Generator, ValueType};


#[pyo3::pyclass(get_all, set_all)]
#[derive(Clone, Debug)]
pub struct RandomConfig {
    pub n_ports_range: (usize, usize),
    pub n_goods_range: (usize, usize),
    pub travel_time_range: (f64, f64),
    pub price_range: (f64, f64),
    pub weight_range: (f64, f64),
    pub capacity_range: (f64, f64),
    pub time_limit_range: (f64, f64),
    pub initial_capital_range: (f64, f64),
    pub visit_cost_range: (f64, f64),
    pub travel_cost_range: (f64, f64),
    pub value_type: String,
    pub max_value: f64,
}

#[pyo3::pymethods]
impl RandomConfig {
    #[new]
    #[pyo3(signature = (
        n_ports_range=(5, 10),
        n_goods_range=(3, 6),
        travel_time_range=(1.0, 10.0),
        price_range=(10.0, 100.0),
        weight_range=(1.0, 5.0),
        capacity_range=(50.0, 200.0),
        time_limit_range=(50.0, 200.0),
        initial_capital_range=(100.0, 500.0),
        visit_cost_range=(5.0, 20.0),
        travel_cost_range=(0.1, 1.0),
        value_type="float".to_string(),
        max_value=100.0
    ))]
    pub fn new(
        n_ports_range: (usize, usize),
        n_goods_range: (usize, usize),
        travel_time_range: (f64, f64),
        price_range: (f64, f64),
        weight_range: (f64, f64),
        capacity_range: (f64, f64),
        time_limit_range: (f64, f64),
        initial_capital_range: (f64, f64),
        visit_cost_range: (f64, f64),
        travel_cost_range: (f64, f64),
        value_type: String,
        max_value: f64,
    ) -> Self {
        Self {
            n_ports_range,
            n_goods_range,
            travel_time_range,
            price_range,
            weight_range,
            capacity_range,
            time_limit_range,
            initial_capital_range,
            visit_cost_range,
            travel_cost_range,
            value_type,
            max_value,
        }
    }
}

impl Default for RandomConfig {
    fn default() -> Self {
        Self {
            n_ports_range: (3, 7),
            n_goods_range: (3, 6),
            travel_time_range: (1.0, 10.0),
            price_range: (10.0, 100.0),
            weight_range: (1.0, 5.0),
            capacity_range: (50.0, 200.0),
            time_limit_range: (50.0, 200.0),
            initial_capital_range: (100.0, 500.0),
            visit_cost_range: (5.0, 20.0),
            travel_cost_range: (0.1, 1.0),
            value_type: "float".to_string(),
            max_value: 100.0,
        }
    }
}

#[pyo3::pyfunction]
#[pyo3(signature = (config, seed=None))]
pub fn generate_random_instance(config: &RandomConfig, seed: Option<u64>) -> Instance {
    let value_type = ValueType::try_from(config.value_type.clone()).unwrap_or(ValueType::Float);
    let mut rng = Generator::new(seed, value_type, config.max_value);

    let n_ports = rng.random_range(config.n_ports_range.0 as f64, config.n_ports_range.1 as f64) as usize;
    let n_goods = rng.random_range(config.n_goods_range.0 as f64, config.n_goods_range.1 as f64) as usize;

    let mut travel_time = rng.random_symmetric_matrix_diagonal_0((n_ports, n_ports));
    for i in 0..n_ports {
        for j in 0..n_ports {
            if i != j {
                travel_time[i][j] = rng.random_range(config.travel_time_range.0, config.travel_time_range.1);
                travel_time[j][i] = travel_time[i][j];
            }
        }
    }

    let weight = (0..n_goods)
        .map(|_| rng.random_range(config.weight_range.0, config.weight_range.1))
        .collect();

    let mut buy_price = vec![vec![0.0; n_goods]; n_ports];
    let mut sell_price = vec![vec![0.0; n_goods]; n_ports];
    let mut buy_cap = vec![vec![0.0; n_goods]; n_ports];
    let mut sell_cap = vec![vec![0.0; n_goods]; n_ports];

    for p in 0..n_ports {
        for g in 0..n_goods {
            let base_price = rng.random_range(config.price_range.0, config.price_range.1);
            buy_price[p][g] = base_price * 1.1; // Buy is more expensive
            sell_price[p][g] = base_price * 0.9; // Sell is cheaper
            buy_cap[p][g] = rng.random_range(10.0, 50.0);
            sell_cap[p][g] = rng.random_range(10.0, 50.0);
        }
    }

    let visit_cost = (0..n_ports)
        .map(|_| rng.random_range(config.visit_cost_range.0, config.visit_cost_range.1))
        .collect();

    Instance {
        n_ports,
        n_goods,
        travel_time,
        travel_cost: rng.random_range(config.travel_cost_range.0, config.travel_cost_range.1),
        weight,
        buy_price,
        sell_price,
        buy_cap,
        sell_cap,
        visit_cost,
        start_port: 0,
        capacity: rng.random_range(config.capacity_range.0, config.capacity_range.1),
        time_limit: rng.random_range(config.time_limit_range.0, config.time_limit_range.1),
        initial_capital: rng.random_range(config.initial_capital_range.0, config.initial_capital_range.1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generation() {
        let config = RandomConfig::default();
        let instance = generate_random_instance(&config, None);
        assert!(instance.n_ports >= config.n_ports_range.0 && instance.n_ports <= config.n_ports_range.1);
        assert!(instance.n_goods >= config.n_goods_range.0 && instance.n_goods <= config.n_goods_range.1);
    }
}
