use std::net::{IpAddr, SocketAddr};

use axum::Router;
use clap::Parser;

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
}

impl App {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub async fn run(&self, router: Router) {
        let args = Args::parse();

        let addr = SocketAddr::new(args.addr, args.port);

        axum_server::bind(addr)
            .serve(router.into_make_service())
            .await
            .unwrap();
    }
}
