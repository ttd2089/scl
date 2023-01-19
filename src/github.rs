
use reqwest::Error;
use serde::Deserialize;

pub struct Context<'a> {
    client: &'a reqwest::Client,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> Context<'a> {

    pub fn new(client: &'a reqwest::Client, owner: &'a str, repo: &'a str) -> Self {
        Self {
            client,
            owner,
            repo,
        }
    }

    pub async fn get_release(&self, release: &str) -> Result<Release, Error> {

        let request_url = format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/{release}",
            owner = self.owner,
            repo = self.repo,
            release = release);

        let response = self.client.get(request_url)
            .header("User-Agent", "scl")
            .send()
            .await?;

        // todo: handle status

        response.json().await
    }

    pub async fn compare_commits(&self, base: &str, head: &str) -> Result<CommitComparison, Error> {

        let request_url = format!(
            "https://api.github.com/repos/{owner}/{repo}/compare/{basehead}",
            owner = self.owner,
            repo = self.repo,
            basehead = format!(
                "{base}...{head}",
                base = base,
                head = head));

        let response = self.client.get(request_url)
            .header("User-Agent", "scl")
            .send()
            .await?;

        // todo: handle status

        response.json().await
    }
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
    pub target_commitish: String,
}

#[derive(Deserialize, Debug)]
pub struct CommitComparison {
    pub status: String,
    pub commits: Vec<CommitItem>,
}

#[derive(Deserialize, Debug)]
pub struct CommitItem {
    pub commit: Commit,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub message: String,
}
