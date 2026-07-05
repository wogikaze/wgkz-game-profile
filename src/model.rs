use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub majsoul_4ma: MajsoulLevel,
    pub majsoul_3ma: MajsoulLevel,
    pub focus_ms: i64,
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
    pub fn from_raw(major: u32, minor: u32, score: i64) -> Self {
        let mut level = Self {
            major,
            minor,
            score,
            is_soul: major >= 6,
        };

        if let Some(max) = level.max_score() {
            if level.score >= max {
                level = level.next_level();
                level.score = level.starting_score();
            }
        }

        level
    }

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
        const LEVEL_MAX_POINTS: [i64; 15] = [
            20, 80, 200, 600, 800, 1000, 1200, 1400, 2000, 2800, 3200, 3600, 4000, 6000, 9000,
        ];
        if self.is_soul {
            return None;
        }

        let minor = self.minor.clamp(1, 3);
        let idx = (self.major.saturating_sub(1) * 3 + minor - 1) as usize;
        LEVEL_MAX_POINTS.get(idx).copied()
    }

    fn starting_score(&self) -> i64 {
        if self.major == 1 {
            0
        } else {
            self.max_score().map(|max| max / 2).unwrap_or(self.score)
        }
    }

    fn next_level(&self) -> Self {
        let mut major = self.major;
        let mut minor = self.minor + 1;
        if minor > 3 {
            major += 1;
            minor = 1;
        }
        Self {
            major,
            minor,
            score: self.score,
            is_soul: major >= 6,
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
        format!("{}/{}_{}.png", base, players, self.major)
    }
}
