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
    let route_meta = wecordy_api::routes::get_user_by_id(10);
    dbg!(route_meta);
    Ok(())
}
