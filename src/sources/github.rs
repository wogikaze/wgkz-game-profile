use crate::config::Config;
use anyhow::{anyhow, Result};
use chrono::{Datelike, TimeZone, Utc};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct GraphQlResp {
    data: GraphQlData,
}

#[derive(Deserialize)]
struct GraphQlData {
    user: Option<UserNode>,
}

#[derive(Deserialize)]
struct UserNode {
    #[serde(rename = "yesterdayCollection")] yesterday_collection: Coll,
    #[serde(rename = "yearCollection")] year_collection: Coll,
}

#[derive(Deserialize)]
struct Coll {
    #[serde(rename = "totalCommitContributions")] total: i64,
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
    let year_end = Utc
        .from_local_datetime(
            &chrono::NaiveDate::from_ymd_opt(now.year(), 12, 31)
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap(),
        )
        .unwrap();

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
    let parsed: GraphQlResp = serde_json::from_str(&text)?;
    let user = parsed.data.user.ok_or_else(|| anyhow!("github user not found"))?;
    Ok((user.yesterday_collection.total, user.year_collection.total))
}
