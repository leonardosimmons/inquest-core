#![allow(unused)]
use inquest::state::{State, StateResponse};

#[tokio::main]
async fn main() {
    let mut system_state = State::new(10);

    let state = system_state.clone();

    state.insert("Hello", "World Test".into());

    if let Some(val) = system_state.get("Hello") {
        println!("get Value: {:?}", val);
    } else {
        println!("Key not found");
    }
}
