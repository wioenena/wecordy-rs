use anyhow::Result;
use wecordy_rest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;

    Ok(())
}

async fn run() -> Result<()> {
    let token = std::env::var("WECORDY_TOKEN")?;
    let rest = ClientBuilder::new(token).build()?;
    let route_meta = wecordy_api::routes::RouteMeta::get_user_by_id(10);
    let x = route_meta.method();
    let y: reqwest::Method = x.into();

    dbg!(x, y);
    Ok(())
}
