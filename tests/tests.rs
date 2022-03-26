#[cfg(test)]
mod parse {
    use inquest::parse::{Html, Parse};
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn all_links_check() {
        let mut buffer = String::new();
        let mut file = File::open("./temp/test.html").await.unwrap();
        file.read_to_string(&mut buffer).await.unwrap();

        let html = Html::new(buffer);
        let parse = Parse::new(html);
        let links = parse.all_links().await.unwrap();

        assert_eq!(vec!["h2-link", "psub-a1-link", "psub-a2-link"], links);
    }
}

#[cfg(test)]
mod state {
    use inquest::state::{State, StateResponse};

    #[test]
    fn data_retrieval() {
        use std::str;

        let state = State::new(10);
        let copy = state.clone();

        copy.insert("Hello", "World".into());

        let resp = match state.get("Hello") {
            StateResponse::Data(data) => data,
            _ => "".into()
        };

        assert_eq!("World", str::from_utf8(&*resp).unwrap());
    }
}
