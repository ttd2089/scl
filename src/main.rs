mod github;

use clap::{arg, command, Command};
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = command!()
        // todo: support detecting current repo and/or using a config file
        .arg(arg!(--owner <OWNER> "The owner of the GitHub repo").required(true))
        .arg(arg!(--repo <REPO> "The name of the GitHub repo").required(true))
        .arg(arg!(--base <RELEASE_TAG> "The tag name of the GitHub release to use as a base (defaults to latest)"))
        .arg(arg!(--target <REF> "The git ref to generate release info for").default_value("HEAD"))
        // todo: investigate the impact of propagate_version -- do subcommands need to have versions?
        .subcommand(
            Command::new("version")
                .about("Generates the version string")
        )
        .subcommand(
            Command::new("changelog")
                .about("Generates the changelog")
        )
        .arg_required_else_help(true)
        .max_term_width(100)
        .help_expected(true)
        .get_matches();

    let client = reqwest::Client::new();
    let context = github::Context::new(
        &client,
        &matches.get_one::<String>("owner").unwrap(),
        &matches.get_one::<String>("repo").unwrap(),
    );

    let release = matches
        .get_one::<String>("base")
        .map_or("latest".into(), |x| format!("tags/{}", x));

    let release = context.get_release(&release).await?;

    let comparison = context
        .compare_commits(
            &release.target_commitish,
            &matches.get_one::<String>("target").unwrap(),
        )
        .await;

    println!("{:?}", comparison);
    Ok(())
}
