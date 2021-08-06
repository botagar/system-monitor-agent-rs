#![feature(decl_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod api;
mod networking;
mod system;

use std::sync::{Arc,RwLock};

use machine_uid;

use networking::Networking;
use system::System;

fn main() {
    println!("Starting agent on port xxxx");
    eprintln!("Port not defined");

    let initial_system_state = System {
        uid: machine_uid::get().unwrap(),
        networking: Networking::new(),
    };

    let sytem_state: Arc<RwLock<System>> = Arc::new(RwLock::new(initial_system_state));

    api::start(sytem_state);
}
