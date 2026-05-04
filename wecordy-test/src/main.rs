use anyhow::Result;
use std::time::Duration;
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wecordy_api::models::ApiResponse;
use wecordy_api::models::user::User;
use wecordy_rest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;

    Ok(())
}

async fn run() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,wecordy_rest=debug"));

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_env_filter(env_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let token = std::env::var("WECORDY_TOKEN")?;
    let rest = ClientBuilder::new(token)
        .with_timeout(Duration::from_secs(3))
        .with_base_url("https://gateway.wecordy.com/api/v1")
        .build()?;
    let user: ApiResponse<User> = rest
        .get(wecordy_api::routes::get_current_user().path())
        .await?;

    dbg!(user);
    Ok(())
}
