use crate::model::instance::PortId;

#[derive(Clone)]
pub enum OfferGenerationStrategy {
    SingleProductBuyFirstSellLast,
}

#[derive(Clone)]
pub enum TravelTimeGenerationStrategy {
    Even,
}

#[derive(Clone)]
pub struct PathRequest {
    /// Port ids of the ports the path will touch, after starting at 0, no ids should
    /// be repeated. The vector will define a loop, returning to node 0. The
    /// generated instance will have at least path.max() + 1 ports. All paths
    /// begin and end at port 0 (Amsterdam) as such, `path` should not contain 0.
    ///
    /// When more than one path request is requested, the port Ids of the second
    /// path request will be shifted up by the total amount of ports of the previous
    /// path requests, to ensure they do not overlap
    pub(super) path: Vec<PortId>,

    /// Amount of ports to add to the generated instance, on top of the ones
    /// added from `path``, this can be 0.
    pub(super) extra_ports: usize,

    /// Sum of all earnings from buying and selling that will be achieved in the path
    pub(super) earnings: f64,

    /// Strategy that will be used to generate buying and selling prices and limits
    pub(super) offer_generation_strategy: OfferGenerationStrategy,

    pub(super) travel_expense: f64,
    pub(super) travel_time_generation_strategy: TravelTimeGenerationStrategy,
}

pub struct GeneratedPathInstance {
    pub(super) offer: GeneratedOffer,

    pub(super) travel_distance: Vec<Vec<f64>>,
}

pub struct GeneratedOffer {
    pub(super) buy_price: Vec<Vec<f64>>,
    pub(super) buy_cap: Vec<Vec<f64>>,
    pub(super) sell_price: Vec<Vec<f64>>,
    pub(super) sell_cap: Vec<Vec<f64>>,

    pub(super) product_weight: Vec<f64>,
}
