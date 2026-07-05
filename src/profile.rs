use crate::config::Config;
use crate::model::Stats;
use anyhow::{anyhow, Result};
use serde_json::{json, Value};

pub fn build_payload(cfg: &Config, stats: &Stats) -> Value {
    let m4 = &stats.majsoul_4ma;
    let m3 = &stats.majsoul_3ma;
    let icon4 = m4.icon_url(&cfg.widget_icon_base, 4);
    let icon3 = m3.icon_url(&cfg.widget_icon_base, 3);
    tracing::info!("icon4 url: {icon4}");
    tracing::info!("icon3 url: {icon3}");
    tracing::info!("stat_icon url: {}", cfg.widget_stat_icon);

    json!({
        "data": {
            "dynamic": [
                { "type": 3, "name": "stat_icon", "value": { "url": cfg.widget_stat_icon } },
                { "type": 1, "name": "rank", "value": m4.format() },
                { "type": 2, "name": "aa", "value": cfg.widget_aa },
                { "type": 1, "name": "title", "value": "雀魂" },
                { "type": 1, "name": "subtitle_1", "value": format!("四麻：{}", m4.format()) },
                { "type": 1, "name": "subtitle_2", "value": format!("三麻：{}", m3.format()) },
                { "type": 1, "name": "subtitle_3", "value": "3" },
                { "type": 3, "name": "icon_4ma", "value": { "url": icon4 } },
                { "type": 3, "name": "icon_3ma", "value": { "url": icon3 } },
                { "type": 2, "name": "t1", "value": stats.focus_ms },
                { "type": 1, "name": "c1", "value": "Focus yesterday" },
                { "type": 2, "name": "t2", "value": stats.commits_yesterday },
                { "type": 1, "name": "c2", "value": "Commits yesterday" },
                { "type": 2, "name": "t3", "value": stats.commits_year },
                { "type": 1, "name": "c3", "value": "Commits this year" },
                { "type": 2, "name": "t4", "value": stats.atcoder_algo },
                { "type": 1, "name": "c4", "value": "AtCoder rating (algo)" },
                { "type": 2, "name": "t5", "value": stats.atcoder_heuristic },
                { "type": 1, "name": "c5", "value": "AtCoder rating (hue)" },
                { "type": 1, "name": "t6", "value": stats.book },
                { "type": 1, "name": "c6", "value": "Last Book Read" },
            ]
        }
    })
}

pub async fn push_profile(cfg: &Config, stats: &Stats) -> Result<()> {
    let payload = build_payload(cfg, stats);
    let url = format!(
        "https://discord.com/api/v9/applications/{}/users/{}/identities/0/profile",
        cfg.discord_app_id, cfg.discord_user_id
    );
    let client = reqwest::Client::builder()
        .user_agent("DiscordBot (https://github.com/discord/discord-api-docs, 1.0.0)")
        .build()?;
    let resp = client
        .patch(&url)
        .header("Authorization", format!("Bot {}", cfg.discord_bot_token))
        .json(&payload)
        .send()
        .await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(anyhow!("discord profile patch {status}: {text}"));
    }
    tracing::info!("discord profile updated");
    Ok(())
}
