/// # Description
/// 
/// State of a Dutch Merchant Problem instance
#[derive(Clone, Debug)]
pub struct State {
    /// # Description
    /// f_j: capital of the boat at time t
    pub capital: f64,

    /// # Description
    /// I_j: inventory of the boat at time t
    pub inventory: Vec<f64>,
}

impl State {
    /// # Description
    /// 
    /// Creates an initial state for a Dutch Merchant Problem instance
    /// 
    /// # Arguments
    /// 
    /// * `initial_capital` - `f_0` initial capital of the boat
    /// * `n_goods` - `n` number of goods
    pub fn new(initial_capital: f64, n_goods: usize) -> Self {
        Self {
            capital: initial_capital,
            inventory: vec![0.0; n_goods],
        }
    }
}
