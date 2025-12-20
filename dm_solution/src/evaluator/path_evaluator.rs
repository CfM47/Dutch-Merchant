use crate::model::instance::{Instance, PortId};

pub trait PathEvaluator {
    /// Calculates the best profit given an instance and a vector of nodes (ports).
    ///
    /// # Arguments
    ///
    /// * `instance` - The instance of the Dutch Merchant Problem.
    /// * `nodes` - The vector of nodes (ports) to calculate the best profit.
    ///
    /// # Returns
    ///
    /// A tuple containing the best profit and the decisions of buying and selling at each node.
    fn calculate_best_profit(self, instance: &Instance, nodes: &[PortId]) -> (f64, Vec<Vec<f64>>);

    fn name(&self) -> &'static str;
}
