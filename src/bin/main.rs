#![allow(unused)]
use inquest::state::{State, StateResponse};

const DEFAULT_STATE_CAPACITY: usize = 1000;

#[tokio::main]
async fn main() {
    let mut system_state = State::new(DEFAULT_STATE_CAPACITY);

    let state = system_state.clone();

    state.insert("Hello", "This is Test Number VI".into());

    match state.get("Hello") {
        StateResponse::Data(data) => println!("get Value: {:?}", data),
        StateResponse::NotFound => println!("Key not found"),
        _ => println!("Unexpected error has occurred"),
    };
}
