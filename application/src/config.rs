use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: Option<String>,
    pub max_conns: u32,
    pub min_conns: u32,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct Logging {
    pub level: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct ConfigInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct Config {
    #[serde(default)]
    pub info: ConfigInfo,
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub logging: Logging,
}

impl Config {
    pub fn new(location: Option<&str>, env_prefix: &str) -> anyhow::Result<Self> {
        todo!()
    }
}
