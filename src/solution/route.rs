pub struct Route{
    pub cost: f64,
    pub vehicle_id: String,
    pub nodes: Vec<usize>,
}

impl Route{
    pub fn new(vehicle_id: String) -> Route{
        Route{
            cost: 0.0,
            vehicle_id,
            nodes: Vec::new(),
        }
    }
}