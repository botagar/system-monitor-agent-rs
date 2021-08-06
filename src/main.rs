#![feature(decl_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod system;
mod api;

use std::sync::{Arc,RwLock};

use machine_uid;

use system::System;

fn main() {
    println!("Starting agent on port xxxx");

    let initial_system_state = System {
        uid: machine_uid::get().unwrap(),
    };

    let sytem_state: Arc<RwLock<System>> = Arc::new(RwLock::new(initial_system_state));

    api::start(sytem_state);
}
