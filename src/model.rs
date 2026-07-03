use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub majsoul_4ma: MajsoulLevel,
    pub majsoul_3ma: MajsoulLevel,
    pub focus_seconds: i64,
    pub commits_yesterday: i64,
    pub commits_year: i64,
    pub atcoder_algo: i64,
    pub atcoder_heuristic: i64,
    pub book: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MajsoulLevel {
    pub major: u32,
    pub minor: u32,
    pub score: i64,
    pub is_soul: bool,
}

impl MajsoulLevel {
    pub fn name(&self) -> &'static str {
        match self.major {
            1 => "初心",
            2 => "雀士",
            3 => "雀傑",
            4 => "雀豪",
            5 => "雀聖",
            6 => "魂天",
            _ => "不明",
        }
    }

    pub fn max_score(&self) -> Option<i64> {
        match self.major {
            1 => Some(200),
            2 => Some(600),
            3 => Some(1200),
            4 => Some(2400),
            5 => Some(4800),
            _ => None,
        }
    }

    pub fn stars(&self) -> String {
        if self.is_soul || self.minor == 0 {
            String::new()
        } else {
            "★".repeat(self.minor.clamp(1, 3) as usize)
        }
    }

    pub fn format(&self) -> String {
        let stars = self.stars();
        match self.max_score() {
            Some(max) => format!("{}{} {}/{}", self.name(), stars, self.score, max),
            None => format!("{} {}", self.name(), self.score),
        }
    }

    pub fn icon_url(&self, base: &str, players: u32) -> String {
        // URL-encode the base path (contains Japanese chars like 基本情報)
        let encoded = urlencoding::encode(base).replace("%2F", "/");
        format!("{}/{}_{}.png", encoded, players, self.major)
    }
}
