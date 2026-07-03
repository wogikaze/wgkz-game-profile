use crate::config::Config;
use crate::model::MajsoulLevel;
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct PlayerStats {
    #[serde(default)]
    level: Option<LevelRaw>,
}

#[derive(Deserialize)]
struct LevelRaw {
    id: u32,
    score: i64,
}

pub async fn fetch(cfg: &Config, players: u32) -> Result<MajsoulLevel> {
    let pl = if players == 4 { "pl4" } else { "pl3" };
    let mode = if players == 4 {
        &cfg.majsoul_mode_4ma
    } else {
        &cfg.majsoul_mode_3ma
    };
    let start = 1262304000000u64;
    let end = chrono::Utc::now().timestamp_millis() as u64;
    let url = format!(
        "{}/{}/player_stats/{}/{}/{}?mode={}",
        cfg.majsoul_base_url, pl, cfg.majsoul_player_id, start, end, mode
    );

    let client = reqwest::Client::builder()
        .user_agent("wgkz-game-profile")
        .build()?;
    let stats: PlayerStats = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let raw = stats.level.ok_or_else(|| anyhow!("no level in stats"))?;
    let major = raw.id / 100;
    let minor = raw.id % 100;
    Ok(MajsoulLevel {
        major,
        minor,
        score: raw.score,
        is_soul: major >= 6,
    })
}
