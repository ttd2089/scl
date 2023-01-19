mod github;

use clap::Parser;
use reqwest::Error;

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
    let client = reqwest::Client::new();
    let context = github::Context::new(&client, &args.owner, &args.repo);
    
    let release = context.get_release("latest").await?;

    let comparison = context.compare_commits(&release.target_commitish, "HEAD").await;

    println!("{:?}", comparison);
    Ok(())
}
