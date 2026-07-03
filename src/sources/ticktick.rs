use crate::config::Config;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Default, Deserialize)]
struct DistResp {
    #[serde(rename = "tagDurations", default)]
    tag_durations: HashMap<String, i64>,
}

pub async fn fetch_yesterday_focus(cfg: &Config) -> Result<i64> {
    // Use JST (UTC+9) since the user is in Japan and TickTick stats are per local day.
    let jst = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
    let now_jst = chrono::Local::now().with_timezone(&jst);
    let yesterday_jst = now_jst - chrono::Duration::days(1);
    let s = yesterday_jst.format("%Y%m%d").to_string();

    let url = format!(
        "https://api.ticktick.com/api/v2/pomodoros/statistics/dist/{}/{}",
        s, s
    );
    tracing::debug!("ticktick focus: querying date {s}, url {url}");

    let cookie = format!("t={}", cfg.ticktick_cookie);
    let client = reqwest::Client::builder()
        .user_agent("wgkz-game-profile")
        .build()?;
    let resp = client
        .get(&url)
        .header("Cookie", cookie)
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        tracing::warn!("ticktick api {status}: {text}");
        return Ok(0);
    }

    tracing::debug!("ticktick response: {text}");

    let parsed: DistResp = serde_json::from_str(&text).unwrap_or_default();
    let total: i64 = parsed.tag_durations.values().sum();
    tracing::info!("ticktick focus yesterday ({s}): {total} minutes");

    Ok(total)
}
