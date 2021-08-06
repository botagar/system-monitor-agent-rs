use crate::system;
use rocket::{State, get, routes};
use rocket_contrib::json::Json;
use serde_json::{Value, json};
use std::sync::{Arc,RwLock};

#[get("/")]
pub fn get_full_state(state: State<Arc<RwLock<system::System>>>) -> Json<Value> {
    return Json(json!(&*state.read().unwrap()));
}

pub fn start(state: Arc<RwLock<system::System>>) {
    rocket::ignite()
        .manage(state)
        .mount("/", routes![get_full_state]).launch();
}