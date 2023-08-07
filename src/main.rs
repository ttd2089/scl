mod git;
mod options;

use std::{env::args_os, error::Error};

use git::{get_conventional_commits, ConventionalCommit};
use options::get_options;
use serde::Serialize;

fn main() {
    run().unwrap_or_else(|e| eprintln!("{}", e));
}

fn run() -> Result<(), Box<dyn Error>> {
    let opts = get_options(args_os())?;
    let changes = get_conventional_commits(&opts)?;
    let version = get_next_version(&opts.base_version, &changes);

    #[derive(Serialize)]
    struct Output<'a> {
        pub version: &'a str,
        pub changes: &'a Vec<ConventionalCommit>,
    }

    let output = serde_json::to_string(&Output {
        version: &version.to_string(),
        changes: &changes,
    })?;

    println!("{}", output);

    Ok(())
}

fn get_next_version(
    base_version: &semver::Version,
    commits: &Vec<ConventionalCommit>,
) -> semver::Version {
    // todo: Use configuration to map additional types for version changes.
    if base_version.major > 0 && commits.iter().any(|x| x.breaking) {
        return semver::Version::new(base_version.major + 1, 0, 0);
    }
    if commits.iter().any(|x| x.type_ == "feat") {
        return semver::Version::new(base_version.major, base_version.minor + 1, 0);
    }
    if commits.iter().any(|x| x.type_ == "fix") {
        return semver::Version::new(
            base_version.major,
            base_version.minor,
            base_version.patch + 1,
        );
    }
    return base_version.to_owned();
}
