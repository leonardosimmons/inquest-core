#[cfg(test)]
mod parse {
    use inquest::html::{Headers, Html, HtmlTag};
    use inquest::parse::Parse;

    async fn test_file() -> String {
        use tokio::fs::File;
        use tokio::io::AsyncReadExt;

        let mut buffer = String::new();
        let mut file = File::open("tests/test.html").await.unwrap();
        file.read_to_string(&mut buffer).await.unwrap();
        buffer
    }

    #[tokio::test]
    async fn all_links_check() {
        let file = test_file().await;
        let html = Html::new(file);
        let parse = Parse::new(html);
        let links = parse.all_links().await.unwrap();

        assert_eq!(vec!["h2-link", "psub-a1-link", "psub-a2-link"], links);
    }

    #[tokio::test]
    async fn get_title_tags() {
        let file = test_file().await;
        let html = Html::new(file);
        let parse = Parse::new(html);

        let h1_header = match parse.headers(HtmlTag::H1).await.unwrap() {
            Headers::H1(h) => h[0].clone(),
            _ => String::from(""),
        };
        assert_eq!("Welcome To The Test HTML File", h1_header);
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
            _ => "".into(),
        };

        assert_eq!("World", str::from_utf8(&*resp).unwrap());
    }
}
