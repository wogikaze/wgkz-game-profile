use crate::config::Config;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct HistoryEntry {
    #[serde(rename = "NewRating")]
    new_rating: i64,
}

async fn latest_rating(cfg: &Config, heuristic: bool) -> Result<i64> {
    let mut url = format!(
        "https://atcoder.jp/users/{}/history/json",
        cfg.atcoder_user
    );
    if heuristic {
        url.push_str("?contestType=heuristic");
    }
    let client = reqwest::Client::builder()
        .user_agent("wgkz-game-profile")
        .build()?;
    let history: Vec<HistoryEntry> = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(history.last().map(|e| e.new_rating).unwrap_or(0))
}

pub async fn fetch_ratings(cfg: &Config) -> Result<(i64, i64)> {
    let (algo, hue) = tokio::try_join!(latest_rating(cfg, false), latest_rating(cfg, true))?;
    Ok((algo, hue))
}
