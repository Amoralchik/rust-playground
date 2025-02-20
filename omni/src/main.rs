use clap::Parser;
use reqwest;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    url: String,
}

#[tokio::main] // Or #[async_std::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let response = reqwest::get(args.url).await?;
    let body = response.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
