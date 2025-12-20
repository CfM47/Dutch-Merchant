use crate::model::instance::{GoodId, PortId};

#[derive(Clone, Debug)]
pub struct Interval {
    pub product: GoodId,
    pub buy_port_index: usize,
    pub sell_port_index: usize,
}

impl Interval {
    pub fn new(product: GoodId, buy_port_index: usize, sell_port_index: usize) -> Self {
        Self {
            product,
            buy_port_index,
            sell_port_index,
        }
    }
}
