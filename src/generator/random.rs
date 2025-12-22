use crate::{
    Instance,
    generator::helpers::{Generator, ValueType},
};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyfunction]
pub fn generate_instance(
    seed: Option<u64>,
    n_ports: usize,
    n_goods: usize,
    value_type: String,
    max_value: f64,
) -> PyResult<String> {
    let value_type = ValueType::try_from(value_type);
    if value_type.is_err() {
        return Err(PyValueError::new_err("Unsupported Value Type"));
    }
    let value_type = value_type.unwrap();

    if max_value < 0_f64 {
        return Err(PyValueError::new_err("Max value must be greater than 0"));
    }

    let mut rng = Generator::new(seed, value_type, max_value);

    let instance = Instance {
        n_ports,
        n_goods,
        travel_time: rng.random_symmetric_matrix_diagonal_0((n_ports, n_ports)),
        travel_cost: rng.random(),
        weight: rng.random_vector(n_goods),
        buy_price: rng.random_matrix((n_ports, n_goods)),
        sell_price: rng.random_matrix((n_ports, n_goods)),
        buy_cap: rng.random_matrix((n_ports, n_goods)),
        sell_cap: rng.random_matrix((n_ports, n_goods)),
        visit_cost: rng.random_vector(n_ports),
        start_port: 0,
        capacity: rng.random(),
        time_limit: rng.random(),
        initial_capital: rng.random(),
    };

    Ok(serde_json::to_string(&instance).unwrap())
}
