#[cfg(test)]
mod tests {
    #[test]
    fn state_data_retrieval() {
        use inquest::state::{State, StateResponse};

        let state = State::new(10);
        let copy = state.clone();

        copy.insert("Hello", "World".into());

        let resp = match state.get("Hello") {
            StateResponse::Data(data) => data,
            _ => bytes::Bytes::from("")
        };

        assert_eq!(bytes::Bytes::from("World"), resp);
    }
}
