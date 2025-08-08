use crate::common;
use std::net::{IpAddr, SocketAddr};

use axum::Router;
use clap::Parser;
use config::{Environment, File};
use sea_orm::DatabaseConnection;

use crate::{AppState, app_router};

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,
    #[arg(short, long, default_value = "0.0.0.0")]
    pub addr: IpAddr,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

pub struct App<T: common::Config> {
    pub name: String,
    pub version: String,
    config: T,
    addr: SocketAddr,
}

impl<T: common::Config> App<T> {
    pub async fn new(name: &str, version: &str) -> anyhow::Result<Self> {
        let args = Args::parse();
        let config =
            Self::build_config(args.clone().config.as_str(), name.to_uppercase().as_str()).await?;
        let addr = SocketAddr::new(args.addr, args.port);

        Ok(Self {
            name: name.to_string(),
            version: version.to_string(),
            config,
            addr,
        })
    }

    pub async fn run(&self, sub_routes: Router<AppState>) -> anyhow::Result<()> {
        let tracing_provider = {
            let provider = common::setup_tracing(
                self.name.clone(),
                self.version.clone(),
                self.config.clone().tracing,
            )?;

            provider
        };

        let state = AppState::new(self.db());

        let router = app_router::create_router(
            state,
            sub_routes,
            self.config.server.timeout_secs.clone(),
            self.config.server.enable_swagger.clone(),
        );

        axum_server::bind(self.addr.clone())
            .serve(router.into_make_service())
            .await
            .unwrap();

        if let Some(tracing_provider) = tracing_provider {
            common::shutdown_opentelemetry(tracing_provider)?;
        }

        Ok(())
    }

    pub fn config(&self) -> T
    where
        T: common::Config,
    {
        self.config.clone()
    }

    pub fn db(&self) -> DatabaseConnection {
        self.db.clone()
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr.clone()
    }

    async fn build_config(location: &str, env_prefix: &str) -> anyhow::Result<T>
    where
        T: common::Config,
    {
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
