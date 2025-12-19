pub type PortId = usize;
pub type GoodId = usize;


/// # Description 
/// 
/// Dutch Merchant Problem instance

#[derive(Clone, Debug)]
pub struct Instance {
    /// ports amount
    pub n_ports: usize,

    /// goods amount
    pub n_goods: usize,

    /// t(u,v): tiempo de viaje de u a v
    /// u and v are bound by n_ports
    pub travel_time: Vec<Vec<f64>>,

    /// w(m): weight of good m
    /// m is bound by n_goods
    pub weight: Vec<f64>,

    /// p+(v,m): buy price of good m at port v
    /// v is bound by n_ports
    /// m is bound by n_goods
    pub buy_price: Vec<Vec<f64>>,

    /// p-(v,m): sell price of good m at port v
    /// v is bound by n_ports
    /// m is bound by n_goods
    pub sell_price: Vec<Vec<f64>>,

    /// c+(v,m): buy stock of good m at port v
    /// v is bound by n_ports
    /// m is bound by n_goods
    pub buy_cap: Vec<Vec<f64>>,

    /// c-(v,m): sell stock of good m at port v
    /// v is bound by n_ports
    /// m is bound by n_goods
    pub sell_cap: Vec<Vec<f64>>,

    /// S(v): cost of visiting port v
    /// v is bound by n_ports
    pub visit_cost: Vec<f64>,

    /// v_0: Initial port
    pub start_port: PortId,
    
    /// B: Boat capacity
    pub capacity: f64,
    
    /// T: Time limit
    pub time_limit: f64,

    /// f_0: Initial capital
    pub initial_capital: f64,
}
