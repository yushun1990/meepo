use application::app::App;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().route("/hello", get(hello));
    App::new("EDGE", "0.1.0").run(router).await?;
    println!("Finished!");

    Ok(())
}

async fn hello() -> &'static str {
    "Hello world!"
}
