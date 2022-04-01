use inquest::cli::Cli;
use inquest::html::Html;
use inquest::parse::Parse;
use inquest::probe::Probe;

#[tokio::main]
async fn main() {
    let cli = Cli::new();

    let result = match cli.path.capacity() == 0 {
        true => {
            let html = Parse::<Html>::from_url(&cli.url[..]).await;
            Probe::html(&cli, html)
        }
        false => {
            let file = get_file(cli.path.to_str().unwrap()).await;
            let parse = Parse::<Html>::from(file.clone());

            Probe::html(&cli, parse)
        }
    }
    .await;

    println!("result: {:?}", result);
}

async fn get_file(path: &str) -> String {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    let mut buffer = String::new();
    let mut file = File::open(path).await.unwrap();
    file.read_to_string(&mut buffer).await.unwrap();
    buffer
}
