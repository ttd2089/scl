mod github;

use std::collections::HashMap;

use clap::{arg, command, Command};
use conventional::Simple;
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
        matches.get_one::<String>("owner").unwrap(),
        matches.get_one::<String>("repo").unwrap(),
    );

    let release = matches
        .get_one::<String>("base")
        .map_or("latest".into(), |x| format!("tags/{}", x));

    let release = context.get_release(&release).await?;

    let comparison = context
        .compare_commits(
            &release.target_commitish,
            matches.get_one::<String>("target").unwrap(),
        )
        .await?;

    let commits = comparison
        .commits
        .iter()
        .filter_map(|x| conventional::Commit::new(&x.commit.message).ok())
        .filter(is_relevant_commit)
        .collect::<Vec<_>>();

    match matches.subcommand() {
        Some(("version", _)) => println!("You typed version :D"),
        Some(("changelog", _)) => print_changelog(&commits),
        _ => unreachable!("Oh nooo"),
    };

    Ok(())
}

fn is_relevant_commit(commit: &conventional::Commit) -> bool {
    commit.type_() == "feat" || commit.type_() == "fix"
}

fn print_changelog(commits: &Vec<conventional::Commit>) {
    let mut categories: HashMap<String, Vec<&conventional::Commit>> = HashMap::new();

    for commit in commits {
        let key = if commit.breaking() {
            "breaking".to_owned()
        } else {
            commit.type_().to_owned()
        };

        categories
            .entry(key)
            .and_modify(|x| x.push(commit))
            .or_insert_with(Vec::new);
    }

    for (category, title) in [
        ("breaking", "BREAKING CHANGES"),
        ("feat", "Features"),
        ("fix", "Bug Fixes"),
    ] {
        print_category(&categories, category, title);
        println!();
    }
}

fn print_category(
    hash_map: &HashMap<String, Vec<&conventional::Commit>>,
    category_name: &str,
    title: &str,
) {
    if let Some(breaking) = hash_map.get(category_name) {
        println!("{}:\n", title);
        for commit in breaking.iter() {
            println!("{subject}", subject = commit.description())
        }
    }
}
