use std::{error::Error, ffi::OsString, ops::Deref, sync::Arc, vec};

use crate::cwrap::{Command, Cwrapper};

pub(crate) fn new<Iter, Item>() -> Box<dyn Command<Iter, Item>>
where
    Iter: IntoIterator<Item = Item>,
    Item: Into<OsString> + Clone,
{
    Box::new(Cwrapper::new(
        build_scl_command,
        parse_scl_options,
        build_scl_context,
        Some(do_scl),
    ))
}

fn build_scl_command() -> clap::Command {
    clap::command!()
        .arg(
            clap::arg!(--base <BASE_REF> "The git ref to compare the target ref against.")
                .required(true),
        )
        .arg(
            clap::arg!(--target <TARGET_REF> "The git ref to generate release info for.")
                .default_value("HEAD"),
        )
        .propagate_version(true)
        .arg_required_else_help(true)
        .max_term_width(100)
}

struct SclOptions {
    pub base: String,
    pub target: String,
}

impl SclOptions {
    fn new(base: &str, target: &str) -> SclOptions {
        SclOptions {
            base: base.to_string(),
            target: target.to_string(),
        }
    }
}

fn parse_scl_options(matches: &clap::ArgMatches) -> SclOptions {
    let base = matches.get_one::<String>("base").unwrap();
    let target = matches.get_one::<String>("target").unwrap();
    SclOptions::new(&base, &target)
}

struct SclContext<'a> {
    repo: git2::Repository,
    commits: Vec<conventional::Commit<'a>>,
}

fn build_scl_context<'a>(opts: SclOptions) -> Result<SclContext<'a>, Box<dyn Error>> {
    // let repo = Arc::new(git2::Repository::discover(".")?);
    let repo = git2::Repository::discover(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_range(&format!("{}..{}", opts.base, opts.target))?;

    let oids = revwalk.collect::<Result<Vec<git2::Oid>, _>>()?;
    // let mut commits = Vec::new();
    for oid in oids {
        repo.find_commit(oid)?;
        // repo.find_commit(oid);
    }

    let mut conv_commits = vec!();

    let mut ctx = SclContext {
        repo,
        commits: conv_commits,
    };

    // let mut messages = Vec::new();

    // for commit in commits {
    //     match commit.message() {
    //         Some(x) => messages.push(Box::new(x.to_owned())),
    //             // .push(Box::new(conventional::Commit::new(x.to_owned()).unwrap())),
    //         _ => {
    //             return Err(format!(
    //                 "invalid commit {}: message it not a valid UTF-8 atring",
    //                 commit.id()
    //             )
    //             .into())
    //         }
    //     }
    // }

    // ctx.commits.push(Box::new(conventional::Commit::new(&messages[0])?.to_owned()));
    Ok(ctx)

    // let commits = revwalk
    //     .map(move |x| match x {
    //         Ok(oid) => repo.find_commit(oid),
    //         Err(e) => Err(e),
    //     })
    //     .collect::<Result<Vec<git2::Commit>, _>>()?;

    // let conventional_commits = commits
    //     .iter()
    //     .map(move |x| match x.message() {
    //         // A subject with trailing newlines is technically not a valid conventional commit
    //         // because it has an empty body instead of no body -- we can be more lenient than that.
    //         Some(x) => Ok(x.trim_end_matches(&['\r', '\n'])),
    //         _ => Err(format!("invalid commit {}: message it not a valid UTF-8 atring", x.id())),
    //     })
    //     .collect::<Result<Vec<&str>, _>>()?
    //     .iter()
    //     .filter_map(|x| conventional::Commit::new(x).ok())
    // .collect();
}

fn do_scl(context: &SclContext) -> Result<(), Box<dyn Error>> {
    // println!("{:#?}", context.commits);
    Ok(())
}
