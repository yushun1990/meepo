use std::net::{IpAddr, SocketAddr};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "0.0.0.0")]
    addr: IpAddr,
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

pub async fn run() {
    let args = Args::parse();

    let addr = SocketAddr::new(args.addr, args.port);
    println!("Listening on: {}", addr);
}
