use inquest::html::HtmlParser;
use inquest::probe::Probe;

#[tokio::main]
async fn main() {
    let probe = Probe::new().document().buffer(4096).html();

    match probe.from("tests/stackoverflow.html").await {
        Ok(probe) => {
            if let Ok(desc) = probe.descriptions() {
                println!("{:?}", desc);
            } else {
                eprintln!("error: failed to retrieve descriptions");
            }
        }
        Err(err) => eprintln!("error: {}", err.to_string()),
    }
}
