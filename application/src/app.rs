use std::net::{IpAddr, SocketAddr};

use axum::Router;
use clap::Parser;

use crate::{config::Config, tracing::shutdown_opentelemetry};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,
    #[arg(short, long, default_value = "INFO")]
    pub tracing_level: tracing::Level,
    #[arg(short, long, default_value = "0.0.0.0")]
    pub addr: IpAddr,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

pub struct App {
    pub name: String,
    pub version: String,
}

impl App {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    pub async fn run(&self, router: Router) -> anyhow::Result<()> {
        let args = Args::parse();

        let config = Config::new(
            args.config.as_str(),
            self.name.clone().to_uppercase().as_str(),
        )?;

        let tracing_provider = {
            let provider = crate::tracing::setup_tracing(
                self.name.clone(),
                self.version.clone(),
                config.tracing,
            )?;
            // Startup span to ensure at least on span is generated and exported
            let span = tracing::info_span!("Meepo!");
            let _ = span.enter();

            provider
        };

        let db = crate::db::setup_database(config.database.clone()).await?;

        let addr = SocketAddr::new(args.addr, args.port);

        axum_server::bind(addr)
            .serve(router.into_make_service())
            .await
            .unwrap();

        println!("Program exit.");

        if let Some(tracing_provider) = tracing_provider {
            shutdown_opentelemetry(tracing_provider)?;
        }

        Ok(())
    }
}
