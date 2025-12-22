use crate::model::instance::PortId;

/// Description
///
/// Dutch Merchant Problem solution
#[pyo3::pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct Solution {
    /// A route of ports R = (v0, v1, ..., vk, v0)
    pub route: Vec<PortId>,

    /// A function of transaction
    /// q^+_j(m), q^-_j(m): the amount of good m bought, sold at port jth of the route
    /// transactions[j][m]
    pub transactions: Vec<Vec<(f64, f64)>>,
}
