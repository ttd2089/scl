use clap::Parser;
use reqwest::Error;
use serde::Deserialize;

#[derive(Parser)]
struct Cli {

    #[clap(long)]
    owner: String,
    
    #[clap(long)]
    repo: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let args = Cli::parse();

    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest",
        owner = args.owner,
        repo = args.repo);
        
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header("User-Agent", "scl")
        .send()
        .await?;
        
    // todo: handle status
    
    let release: Release = response.json().await?;
    println!("{:?}", release);
    Ok(())
}

#[derive(Deserialize, Debug)]
struct Release {
    name: String,
    tag_name: String,
    target_commitish: String,
}
