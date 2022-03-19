#![allow(unused)]
use inquest::state::{State, StateResponse};

const MAX_STATE_CAPACITY: usize = 999999;

#[tokio::main]
async fn main() {
    let mut system_state = State::new(MAX_STATE_CAPACITY);

    let state = system_state.clone();

    state.insert("Hello", "This is Test Number VII".into());

    match system_state.get("Hello") {
        StateResponse::Data(data) => println!("get Value: {:?}", data),
        StateResponse::NotFound => println!("Key not found"),
        StateResponse::Error(err) => println!("{}", err.msg),
        _ => {}
    };
}
