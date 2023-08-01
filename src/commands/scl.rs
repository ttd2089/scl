use std::error::Error;
use std::ffi::OsString;

use super::{version, changelog};

pub(super) struct SclOptions {
    pub(super) base: String,
    pub(super) target: String,
}

impl SclOptions {
    pub(super) fn new(base: &str, target: &str) -> SclOptions {
        SclOptions{ base: base.to_string(), target: target.to_string() }
    }
}

pub(crate) fn new() -> SclCommand {
    SclCommand{}
}

pub(crate) struct SclCommand;

impl SclCommand {

    pub(crate) fn run<Iter, Item>(&self, itr: Iter) -> Result<(), Box<dyn Error>>
    where
        Iter: IntoIterator<Item = Item>,
        Item: Into<OsString> + Clone, {

        let clap_command = self.build_clap_command();
        let matches = clap_command.try_get_matches_from(itr)?;
        let opts = self.parse_matches(&matches);
        self.execute(&opts, &matches)
    }

    fn build_clap_command(&self) -> clap::Command {
        clap::command!()
            .arg(clap::arg!(--base <BASE_REF> "The git ref to compare the target ref against.").required(true))
            .arg(clap::arg!(--target <TARGET_REF> "The git ref to generate release info for.").default_value("HEAD"))
            .propagate_version(true)
            .arg_required_else_help(true)
            .max_term_width(100)
            .subcommand(version::build_subcommand())
            .subcommand(changelog::build_subcommand())
    }

    fn parse_matches(&self, matches: &clap::ArgMatches) -> SclOptions {
        let base = matches.get_one::<String>("base").unwrap();
        let target = matches.get_one::<String>("target").unwrap();
        SclOptions::new(&base, &target)
    }

    fn execute(&self, opts: &SclOptions, matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {

        let repo = git2::Repository::discover(".")?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_range(&format!("{}..{}", opts.base, opts.target))?;

        let commits = revwalk
            .collect::<Result<Vec<git2::Oid>, _>>()?
            .iter()
            .map(|x| repo.find_commit(*x))
            .collect::<Result<Vec<git2::Commit>, _>>()?;

        let conventional_commits = commits
            .iter()
            .map(|x| match x.message() {
                // A subject with trailing newlines is technically not a valid conventional commit
                // because it has an empty body instead of no body -- we can be more lenient than that.
                Some(x) => Ok(x.trim_end_matches(&['\r', '\n'])),
                _ => Err(format!("invalid commit {}: message it not a valid UTF-8 atring", x.id())),
            })
            .collect::<Result<Vec<&str>, _>>()?
            .iter()
            .filter_map(|x| conventional::Commit::new(x).ok())
            .collect();
        
        match matches.subcommand() {
            Some(("version", matches)) => version::run(matches, &conventional_commits),
            Some(("changelog", matches)) => changelog::run(matches, &conventional_commits),
            Some((x, _)) => unreachable!("unknown subcommand '{}'", x),
            None => unreachable!("no subcommand given"),
        };

        Ok(())
    }
}
