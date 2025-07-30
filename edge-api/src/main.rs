use application::app::App;
use edge_api::domains::hello::api::routes::hello_routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::new("Meepo-Edge", "v0.1.0").await?;

    let sub_routes = hello_routes();
    app.run(sub_routes).await?;

    Ok(())
}
