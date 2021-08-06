use crate::networking::Networking;

#[derive(Debug,Serialize)]
pub struct System {
    pub uid: String,
    pub networking: Networking
}