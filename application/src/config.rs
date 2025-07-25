use config::{Environment, File};
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
pub struct Tracing {
    pub filter_str: Option<String>,
    pub enable_otlp: bool,
    pub otlp_endpoint: Option<String>,
    pub otlp_protocol: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct Config {
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub logging: Logging,
}

impl Config {
    #[allow(unused)]
    pub fn new(location: &str, env_prefix: &str) -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let config = config::Config::builder()
            .add_source(File::with_name(location))
            .add_source(
                Environment::with_prefix(env_prefix)
                    .separator("_")
                    .prefix_separator("__"),
            )
            .build()?;

        let config = config.try_deserialize()?;

        Ok(config)
    }
}
