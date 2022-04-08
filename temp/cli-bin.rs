use inquest::cli::Cli;
use inquest::html::Html;
use inquest::parse::Parse;
use inquest::probe::Probe;
use inquest::utils::Result;

#[tokio::main]
async fn main() {
    let cli = Cli::new();

    let _capacity = cli.path.capacity();

    let task = tokio::spawn(async move {
        process(cli).await.unwrap_or_else(|err| {
            println!("{}", err.to_string());
            Vec::new()
        })
    });

    let result = task.await.unwrap();
    println!("result: {:?}", result);
}

async fn process(cli: Cli) -> Result<Vec<String>> {
    let html = match cli.path.capacity() == 0 {
        true => Parse::<Html>::from_url(&cli.url[..]).await?,
        false => {
            let file = get_file(cli.path.to_str().unwrap())
                .await
                .unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                    String::new()
                });
            Parse::<Html>::from(&file)?
        }
    };

    Ok(Probe::html(html, &cli).await?)
}

async fn get_file(path: &str) -> std::result::Result<String, std::io::Error> {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    let mut buffer = String::new();
    let mut file = File::open(path).await?;
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
