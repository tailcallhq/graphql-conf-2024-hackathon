use anyhow::{bail, Context, Result};
use octocrate::{APIConfig, GitHubAPI, PersonalAccessToken};
use regex::Regex;
use tokio::task::JoinSet;
use tracing::{error, info};

const OWNER: &str = "tailcallhq";
const REPO: &str = "hackathon";
const LEADERBOARD_ISSUE: i64 = 30;

#[derive(Debug)]
struct Score {
    author: String,
    score: u64,
}

async fn run() -> Result<()> {
    let score_regex = Regex::new(r"The score is: \*\*(\d+)\*\*")?;

    let token = PersonalAccessToken::new(
        std::env::var("GITHUB_TOKEN").context("env.GITHUB_TOKEN not found")?,
    );

    // Create a default GitHub API configuration
    let config = APIConfig::with_token(token).shared();

    let api = GitHubAPI::new(&config);

    info!("Request all prs");

    let pull_request = api.pulls.list(OWNER, REPO).send().await?;

    info!("Got prs info");

    let hackathon_prs = pull_request
        .into_iter()
        .filter(|pr| pr.labels.iter().any(|label| label.name == "ci: benchmark"));

    let mut join_set: JoinSet<Result<Score>> = JoinSet::new();

    info!("Check score comments for every pr");

    for pr in hackathon_prs {
        let config = config.clone();
        let score_regex = score_regex.clone();
        join_set.spawn(async move {
            let api = GitHubAPI::new(&config);

            let comments = api
                .issues
                .list_comments(OWNER, REPO, pr.number)
                .send()
                .await?;

            for comment in comments {
                if comment.performed_via_github_app.is_some() {
                    if let Some(body) = comment.body {
                        if let Some(caps) = score_regex.captures(&body) {
                            return Ok(Score {
                                author: format!(
                                    "_{}_",
                                    pr.user.context("Failed to resolve author")?.login
                                ),
                                score: caps[1].parse()?,
                            });
                        }
                    }
                }
            }

            bail!("Failed to infer the score")
        });
    }

    info!("Got scores");

    let mut scores = join_set
        .join_all()
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    scores.push(Score {
        author: "**Tailcall**".to_owned(),
        score: 1000,
    });

    scores.sort_by(|a, b| b.score.cmp(&a.score));

    let mut content = "| Participant | Score |\n|---|---|\n".to_owned();

    for score in scores {
        content.push_str(&format!("|{}|{}|\n", &score.author, score.score));
    }

    let request = octocrate::issues::update::Request::builder()
        .body(content)
        .build();

    api.issues
        .update(OWNER, REPO, LEADERBOARD_ISSUE)
        .body(&request)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(error) = run().await {
        error!("Critical error: {:#}", error);
        panic!("Critical error");
    }
}
