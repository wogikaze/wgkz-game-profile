pub mod atcoder;
pub mod github;
pub mod mahjong;
pub mod ticktick;

use crate::model::Stats;
use anyhow::Result;

pub async fn fetch_all(cfg: &crate::config::Config, book: String) -> Result<Stats> {
    let (m4, m3, focus, (cy, cyear), (algo, hue)) = tokio::try_join!(
        mahjong::fetch(cfg, 4),
        mahjong::fetch(cfg, 3),
        ticktick::fetch_yesterday_focus(cfg),
        github::fetch_commits(cfg),
        atcoder::fetch_ratings(cfg),
    )?;

    Ok(Stats {
        majsoul_4ma: m4,
        majsoul_3ma: m3,
        focus_seconds: focus,
        commits_yesterday: cy,
        commits_year: cyear,
        atcoder_algo: algo,
        atcoder_heuristic: hue,
        book,
    })
}
