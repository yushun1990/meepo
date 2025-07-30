use std::net::{IpAddr, SocketAddr};

use axum::Router;
use clap::Parser;
use sea_orm::DatabaseConnection;

use crate::{AppState, app_router, common};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,
    #[arg(short, long, default_value = "0.0.0.0")]
    pub addr: IpAddr,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

pub struct App {
    pub name: String,
    pub version: String,
    config: common::Config,
    db: DatabaseConnection,
    addr: SocketAddr,
}

impl App {
    pub async fn new(name: &str, version: &str) -> anyhow::Result<Self> {
        let args = Args::parse();
        let config =
            common::Config::new(args.clone().config.as_str(), name.to_uppercase().as_str())?;
        let db = common::setup_database(config.database.clone()).await?;
        let addr = SocketAddr::new(args.addr, args.port);

        Ok(Self {
            name: name.to_string(),
            version: version.to_string(),
            config,
            db,
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

    pub fn config(&self) -> common::Config {
        self.config.clone()
    }

    pub fn db(&self) -> DatabaseConnection {
        self.db.clone()
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr.clone()
    }
}
