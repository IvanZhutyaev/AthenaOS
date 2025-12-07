use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AthenaConfig {
    pub data_dir: PathBuf,
    pub key_store_path: PathBuf,
    pub graph_db_path: PathBuf,
    pub p2p_port: u16,
    pub api_port: u16,
    pub enable_p2p: bool,
}

impl Default for AthenaConfig {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("athena-os");

        Self {
            data_dir: data_dir.clone(),
            key_store_path: data_dir.join("keys.bin"),
            graph_db_path: data_dir.join("graph"),
            p2p_port: 9000,
            api_port: 8080,
            enable_p2p: true,
        }
    }
}

impl AthenaConfig {
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

