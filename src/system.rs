use self::{networking::Networking, processor::Processor};

pub mod processor;
pub mod networking;

#[derive(Debug,Serialize)]
pub struct System {
    pub uid: String,
    pub networking: Networking,
    pub cpu: Processor,
}

impl Default for System {
    fn default() -> Self {
        Self { 
            uid: Default::default(), 
            networking: Default::default(),
            cpu: Processor::default()
        }
    }
}