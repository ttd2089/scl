use std::error::Error;

use git_conventional::FooterSeparator;
use serde::Serialize;

use crate::options::SclOptions;

#[derive(Debug, Serialize)]
pub(super) struct ConventionalCommit {
    pub breaking: bool,
    pub breaking_changes: Vec<String>,
    pub type_: String,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub footers: Vec<ConventionalCommitFooter>,
    pub sha: String,
}

impl ConventionalCommit {
    fn new(oid: &git2::Oid, conventional_commit: &git_conventional::Commit) -> ConventionalCommit {
        ConventionalCommit {
            breaking: conventional_commit.breaking(),
            breaking_changes: Self::get_breaking_changes(conventional_commit),

            type_: conventional_commit.type_().to_string(),
            scope: conventional_commit.scope().map(|x| x.to_string()),
            description: conventional_commit.description().to_owned(),
            body: conventional_commit.body().map(|x| x.to_owned()),
            footers: conventional_commit
                .footers()
                .iter()
                .map(|x| ConventionalCommitFooter::new(x))
                .collect(),
            sha: oid.to_string(),
        }
    }

    fn get_breaking_changes(conventional_commit: &git_conventional::Commit) -> Vec<String> {
        // The value returned by `breaking_description()` can come from the subject or from a BREAKING CHANGE footer,
        // but if both are present then the footer is preferred. This means we can't reliably expose all breaking
        // change descriptions because if one or more BREAKING CHANGE footers are included then don't know whether the
        // subject was meant to indicate a breaking change or not.
        //
        // The conventional commit specification says the subject should be used if both are present and doing so would
        // fix our issue so I opened this PR:
        //
        // https://github.com/crate-ci/git-conventional/pull/45

        if let Some(breaking_description) = conventional_commit.breaking_description() {
            let mut other_breaking_changes = conventional_commit
                .footers()
                .iter()
                .filter(|x| x.breaking() && x.value() != breaking_description)
                .map(|x| x.value().to_owned())
                .collect::<Vec<_>>();

            let mut breaking_changes =
                Vec::<String>::with_capacity(other_breaking_changes.len() + 1);

            breaking_changes.push(breaking_description.to_owned());
            breaking_changes.append(&mut other_breaking_changes);
            return breaking_changes;
        }
        vec![]
    }
}

#[derive(Debug, Serialize)]
pub(super) struct ConventionalCommitFooter {
    pub token: String,
    pub value: String,
    pub breaking: bool,
    separator: String,
}

impl ConventionalCommitFooter {
    fn new(footer: &git_conventional::Footer) -> ConventionalCommitFooter {
        ConventionalCommitFooter {
            token: footer.token().to_string(),
            value: footer.value().to_owned(),
            breaking: footer.breaking(),
            separator: match footer.separator() {
                FooterSeparator::Value => ": ".to_owned(), // FooterSeparator is defined as ":" instead of ": "
                FooterSeparator::Ref => FooterSeparator::Ref.to_string(),
                x => x.to_string(),
            },
        }
    }
}

impl ToString for ConventionalCommitFooter {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.token, self.separator, self.value)
    }
}

pub(super) fn get_conventional_commits(
    opts: &SclOptions,
) -> Result<Vec<ConventionalCommit>, Box<dyn Error>> {
    let repo = git2::Repository::discover(&opts.repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_range(&format!("{}..{}", opts.base_ref, opts.target_ref))?;

    let mut conventional_commits = Vec::<ConventionalCommit>::new();

    for oid_result in revwalk {
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;
        let message = commit.message().ok_or(format!(
            "commit message for {} is not a valid UTF-8 string",
            oid
        ))?;

        let conventional_commit = git_conventional::Commit::parse(message);

        match (conventional_commit, opts.strict) {
            (Ok(x), _) => {
                conventional_commits.push(ConventionalCommit::new(&oid, &x));
            }
            (Err(e), true) => Err(format!("found unconventional commit '{}': {}", oid, e))?,
            _ => {}
        }
    }

    Ok(conventional_commits)
}
