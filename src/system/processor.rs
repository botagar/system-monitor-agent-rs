#[derive(Debug,Serialize)]
pub struct Processor {
    pub vendor: String,
    pub family: String,
    pub load_percent: f64,
    pub details: Details,
}

impl Default for Processor {
    fn default() -> Self {
        Self { 
            vendor: String::from("Unknown"),
            family: String::from("Unknown"),
            load_percent: 0.0,
            details: Details::default(),
        }
    }
}

#[derive(Debug,Serialize)]
pub struct Details {
    pub family_id: u8,
    pub extended_family_id: u8,
    pub model_id: u8,
    pub extended_model_id: u8,
}

impl Default for Details {
    fn default() -> Self {
        Self {
            family_id: 0,
            extended_family_id: 0,
            model_id: 0,
            extended_model_id: 0,
        }
    }
}