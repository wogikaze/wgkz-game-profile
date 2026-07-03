use crate::config::Config;
use anyhow::{anyhow, Result};
use chrono::{Datelike, TimeZone, Utc};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct GraphQlResp {
    #[serde(default)]
    data: Option<GraphQlData>,
    #[serde(default)]
    errors: Option<Vec<GraphQlError>>,
}

#[derive(Deserialize)]
struct GraphQlError {
    message: String,
}

#[derive(Deserialize)]
struct GraphQlData {
    #[serde(default)]
    user: Option<UserNode>,
}

#[derive(Deserialize)]
struct UserNode {
    #[serde(rename = "yesterdayCollection", default)]
    yesterday_collection: Option<Coll>,
    #[serde(rename = "yearCollection", default)]
    year_collection: Option<Coll>,
}

#[derive(Deserialize)]
struct Coll {
    #[serde(rename = "totalCommitContributions", default)]
    total: i64,
}

pub async fn fetch_commits(cfg: &Config) -> Result<(i64, i64)> {
    let now = Utc::now();
    let yesterday = now - chrono::Duration::days(1);
    let yesterday_start = Utc
        .from_local_datetime(&yesterday.date_naive().and_hms_opt(0, 0, 0).unwrap())
        .unwrap();
    let yesterday_end = yesterday_start + chrono::Duration::days(1);
    let year_start = Utc
        .from_local_datetime(
            &chrono::NaiveDate::from_ymd_opt(now.year(), 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        )
        .unwrap();
    // GitHub contributionsCollection: date range must be within one year.
    // Use "now" as the end instead of Dec 31 to stay within limits.
    let year_end = now;

    let q = "query($login:String!,$ys:DateTime!,$ye:DateTime!,$yrs:DateTime!,$yre:DateTime!){\
           user(login:$login){\
             yesterdayCollection:contributionsCollection(from:$ys,to:$ye){totalCommitContributions}\
             yearCollection:contributionsCollection(from:$yrs,to:$yre){totalCommitContributions}\
           }\
         }";

    let body = json!({
        "query": q,
        "variables": {
            "login": cfg.github_user,
            "ys": yesterday_start.to_rfc3339(),
            "ye": yesterday_end.to_rfc3339(),
            "yrs": year_start.to_rfc3339(),
            "yre": year_end.to_rfc3339(),
        }
    });

    tracing::debug!(
        "github graphql: yesterday {}..{}, year {}..{}",
        yesterday_start.to_rfc3339(),
        yesterday_end.to_rfc3339(),
        year_start.to_rfc3339(),
        year_end.to_rfc3339()
    );

    let client = reqwest::Client::builder()
        .user_agent("wgkz-game-profile")
        .build()?;
    let resp = client
        .post("https://api.github.com/graphql")
        .bearer_auth(&cfg.github_token)
        .json(&body)
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!("github graphql {status}: {text}"));
    }

    let parsed: GraphQlResp = serde_json::from_str(&text)
        .map_err(|e| anyhow!("github graphql parse error: {e}; body: {text}"))?;

    if let Some(errs) = &parsed.errors {
        let msgs: Vec<_> = errs.iter().map(|e| e.message.clone()).collect();
        return Err(anyhow!("github graphql errors: {}", msgs.join("; ")));
    }

    let user = parsed
        .data
        .and_then(|d| d.user)
        .ok_or_else(|| anyhow!("github user not found; body: {text}"))?;

    let yesterday_total = user.yesterday_collection.map(|c| c.total).unwrap_or(0);
    let year_total = user.year_collection.map(|c| c.total).unwrap_or(0);

    tracing::info!(
        "github commits: yesterday={yesterday_total}, year={year_total}"
    );

    Ok((yesterday_total, year_total))
}
