use config::{Environment, File};
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: Option<String>,
    #[serde(default)]
    pub max_conns: u32,
    #[serde(default)]
    pub min_conns: u32,
    #[serde(default)]
    pub conn_timeout_millis: u64,
    #[serde(default)]
    pub idle_timeout_millis: u64,
    pub enable_logging: Option<bool>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct Tracing {
    pub filter_str: Option<String>,
    #[serde(default)]
    pub enable_otlp: bool,
    pub otlp_endpoint: Option<String>,
    pub otlp_protocol: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub timeout_secs: u64,
    pub enable_swagger: bool,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[allow(unused)]
pub struct Config {
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub tracing: Tracing,
    #[serde(default)]
    pub server: Server,
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

impl Default for Server {
    fn default() -> Self {
        Self {
            timeout_secs: Default::default(),
            enable_swagger: true,
        }
    }
}
