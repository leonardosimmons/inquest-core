use inquest::cli::Cli;
use inquest::html::Html;
use inquest::parse::Parse;
use inquest::probe::Probe;

#[tokio::main]
async fn main() {
    let cli = Cli::new();

    let html = match cli.path.capacity() == 0 {
        true => {
            Parse::<Html>::from_url(&cli.url[..]).await.unwrap_or_else(|err| {
                println!("{}", err.to_string());
                Parse::from(String::from(""))
            })
        }
        false => {
            let file = get_file(cli.path.to_str().unwrap()).await.unwrap_or_else(|err| {
                println!("{}", err.to_string());
                String::new()
            });
            Parse::<Html>::from(file)
        }
    };

    let result = Probe::html(html, &cli).await.unwrap_or_else(|err| {
        println!("{}", err.to_string());
        Vec::new()
    });

    println!("result: {:?}", result);
}

async fn get_file(path: &str) -> Result<String, std::io::Error> {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    let mut buffer = String::new();
    let mut file = File::open(path).await?;
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
