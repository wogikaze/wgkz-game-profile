use crate::config::Config;
use crate::model::MajsoulLevel;
use anyhow::Result;
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

#[derive(Deserialize)]
struct ApiError {
    #[serde(default)]
    error: Option<String>,
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
    let resp = client.get(&url).send().await?;
    let status = resp.status();
    let text = resp.text().await?;

    // 404 with {"error":"id_not_found"} means no ranked games played yet.
    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<ApiError>(&text) {
            if err.error.as_deref() == Some("id_not_found") {
                tracing::warn!("majsoul {} player_stats: no data (not played yet)", pl);
                return Ok(MajsoulLevel {
                    major: 1,
                    minor: 0,
                    score: 0,
                    is_soul: false,
                });
            }
        }
        anyhow::bail!("majsoul {} api {}: {}", pl, status, text);
    }

    let stats: PlayerStats = serde_json::from_str(&text)?;
    let raw = match stats.level {
        Some(l) => l,
        None => {
            tracing::warn!("majsoul {} stats: no level field", pl);
            return Ok(MajsoulLevel {
                major: 1,
                minor: 0,
                score: 0,
                is_soul: false,
            });
        }
    };
    let major = raw.id / 100;
    let minor = raw.id % 100;
    Ok(MajsoulLevel {
        major,
        minor,
        score: raw.score,
        is_soul: major >= 6,
    })
}
