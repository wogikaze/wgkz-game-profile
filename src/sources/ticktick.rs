use crate::config::Config;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct DistResp {
    #[serde(rename = "tagDurations", default)]
    tag_durations: HashMap<String, i64>,
}

pub async fn fetch_yesterday_focus(cfg: &Config) -> Result<i64> {
    let d = chrono::Local::now() - chrono::Duration::days(1);
    let s = d.format("%Y%m%d").to_string();

    let url = format!(
        "https://api.ticktick.com/api/v2/pomodoros/statistics/dist/{}/{}",
        s, s
    );
    let cookie = format!("t={}", cfg.ticktick_cookie);
    let client = reqwest::Client::builder()
        .user_agent("wgkz-game-profile")
        .build()?;
    let resp: DistResp = client
        .get(&url)
        .header("Cookie", cookie)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp.tag_durations.values().sum())
}
