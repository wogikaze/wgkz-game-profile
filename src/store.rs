use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub book: String,
}

impl State {
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let s = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&s).unwrap_or_default())
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
