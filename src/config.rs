use anyhow::{Context, Result};
use std::env;

#[derive(Clone)]
pub struct Config {
    pub discord_app_id: u64,
    pub discord_bot_token: String,
    pub discord_user_id: u64,

    pub majsoul_player_id: String,
    pub majsoul_base_url: String,
    pub majsoul_mode_4ma: String,
    pub majsoul_mode_3ma: String,

    pub github_token: String,
    pub github_user: String,

    pub ticktick_cookie: String,

    pub atcoder_user: String,

    pub widget_stat_icon: String,
    pub widget_icon_base: String,
    pub widget_aa: i64,
    pub widget_cron: String,

    pub state_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let get = |k: &str| env::var(k).context(format!("missing env {k}"));
        Ok(Self {
            discord_app_id: get("DISCORD_APP_ID")?.parse().context("DISCORD_APP_ID")?,
            discord_bot_token: get("DISCORD_BOT_TOKEN")?,
            discord_user_id: get("DISCORD_USER_ID")?.parse().context("DISCORD_USER_ID")?,

            majsoul_player_id: get("MAJSOUL_PLAYER_ID")?,
            majsoul_base_url: env::var("MAJSOUL_BASE_URL")
                .unwrap_or_else(|_| "https://5-data.amae-koromo.com/api/v2".into()),
            majsoul_mode_4ma: env::var("MAJSOUL_MODE_4MA")
                .unwrap_or_else(|_| "16.12.9.15.11.8".into()),
            majsoul_mode_3ma: env::var("MAJSOUL_MODE_3MA")
                .unwrap_or_else(|_| "26.24.22.25.23.21".into()),

            github_token: get("GITHUB_TOKEN")?,
            github_user: get("GITHUB_USER")?,

            ticktick_cookie: get("TICKTICK_COOKIE")?,

            atcoder_user: get("ATCODER_USER")?,

            widget_stat_icon: env::var("WIDGET_STAT_ICON").unwrap_or_else(|_| {
                "https://i.gyazo.com/7fe75df2979682c5d38cdbe17a737817.png".into()
            }),
            widget_icon_base: env::var("WIDGET_ICON_BASE").unwrap_or_else(|_| {
                "https://cdn.wikiwiki.jp/to/w/nya/基本情報/::attach".into()
            }),
            widget_aa: env::var("WIDGET_AA")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(360000),
            widget_cron: env::var("WIDGET_CRON")
                .unwrap_or_else(|_| "0 */30 * * * *".into()),

            state_path: env::var("STATE_PATH").unwrap_or_else(|_| "data/state.json".into()),
        })
    }
}
