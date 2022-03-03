#[derive(Debug,Serialize)]
pub struct Processor {
    pub name: String,
    pub load: f64,
}

impl Default for Processor {
    fn default() -> Self {
        Self { 
            name: String::from("Unknown"),
            load: 0.0,
        }
    }
}