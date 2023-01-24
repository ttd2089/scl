mod github;

use std::collections::HashMap;

use clap::{arg, command, Command};
use conventional::Simple;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut cmd = command!()
        // todo: validate base is a version or "latest"
        // todo: support detecting current repo and/or using a config file
        .arg(arg!(--owner <OWNER> "The owner of the GitHub repo").required(true))
        .arg(arg!(--repo <REPO> "The name of the GitHub repo").required(true))
        .arg(arg!(--base <RELEASE_TAG> "The tag name of the GitHub release to use as a base (defaults to latest)")
            .required(true)
            .value_parser(VersionTagParser{}))
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
        .max_term_width(100)
        .help_expected(true);

    // note: Calling render_help after get_matches doesn't work because it attempts to borrow a moved value. I don't
    // understand why render_help borrows and get_matches moves. The intention here is to have the default subcommand
    // be help but I didn't find anything in clap about default subcommands so here we are.
    let help_text = cmd.render_help();

    let matches = cmd.get_matches();

    let client = reqwest::Client::new();
    let context = github::Context::new(
        &client,
        matches.get_one::<String>("owner").unwrap(),
        matches.get_one::<String>("repo").unwrap(),
    );

    // note: This doesn't actually make sense. The parser for --base should confirm that we can get a version from the
    // string, not actually convert to a version. Since we may or may not have a 'v' prefix it's not safe to assume we
    // do; I'm just getting away with it because we're only testing against peer. I left this in just because I wanted
    // to show Insomniak47 the impl of VersionTagParser from before I realized this was a mistake.
    let release = matches
        .get_one::<semver::Version>("base")
        .map_or("latest".into(), |x| format!("tags/v{}", x.to_string()));

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
        Some(("version", _)) => print_version(matches.get_one::<semver::Version>("base").unwrap(), &commits),
        Some(("changelog", _)) => print_changelog(&commits),
        None => {
            println!("{}", help_text);
        }
        _ => unreachable!("Oh nooo"),
    };

    Ok(())
}

fn is_relevant_commit(commit: &conventional::Commit) -> bool {
    commit.type_() == "feat" || commit.type_() == "fix"
}

fn print_version(base: &semver::Version, commits: &Vec<conventional::Commit>) {
    println!("{}", get_version(base, commits));
}

fn get_version(base: &semver::Version, commits: &Vec<conventional::Commit>) -> semver::Version {
    if commits.iter().any(|x| x.breaking()) {
        return semver::Version::new(base.major + 1, 0, 0);
    }
    if commits.iter().any(|x| x.type_() == "feat") {
        return semver::Version::new(base.major, base.minor + 1, 0);
    }
    if commits.iter().any(|x| x.type_() == "fix") {
        return semver::Version::new(base.major, base.minor, base.patch + 1);
    }
    base.to_owned()
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

#[derive(Clone)]
struct VersionTagParser;

impl VersionTagParser {
    fn make_error(cmd: &clap::Command, arg: Option<&clap::Arg>, value: &str) -> clap::Error {
        let mut err = clap::Error::new(clap::error::ErrorKind::ValueValidation).with_cmd(cmd);
        if let Some(arg) = arg {
            err.insert(clap::error::ContextKind::InvalidArg, clap::error::ContextValue::String(arg.to_string()));
        }
        err.insert(clap::error::ContextKind::InvalidValue, clap::error::ContextValue::String(value.to_owned()));
        err
    }
}

impl clap::builder::TypedValueParser for VersionTagParser {

    type Value = semver::Version;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^v?(?P<version>\d+\.\d+\.\d+)$").unwrap();
        }

        let arg_value = value.to_string_lossy();

        let result = RE
            .captures(&arg_value)
            .map(|x| x.name("version"))
            .flatten()
            .map(|x| x.as_str())
            .map(|x| semver::Version::parse(x))
            .ok_or_else(|| VersionTagParser::make_error(cmd, arg, &arg_value))
            .map(|x| {
                x.map_err(|e| {
                    let mut err = VersionTagParser::make_error(cmd, arg, &arg_value);
                    err.insert(clap::error::ContextKind::Custom, clap::error::ContextValue::String(e.to_string()));
                    err
                })
            });

        match result {
            Ok(Ok(x)) => Ok(x),
            Ok(x) => x,
            Err(x) => Err(x),
        }
    }

}
