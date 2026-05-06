use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use wecordy_rest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug,wecordy_rest=debug,wecordy_ws=debug"));

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_env_filter(env_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    run().await?;

    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn run() -> Result<()> {
    let token: Arc<str> = Arc::from(std::env::var("WECORDY_TOKEN")?);
    let _rest = ClientBuilder::new(Arc::clone(&token))
        .with_timeout(Duration::from_secs(3))
        .with_base_url("https://gateway.wecordy.com/api/v1")
        .build()?;

    let intents: Arc<Vec<String>> = Arc::new(
        ["Servers", "ServerMessages", "MessageContent"]
            .iter()
            .map(|&i| i.to_owned())
            .collect::<Vec<String>>(),
    );
    let websocket = wecordy_ws::WebSocket::new(
        Arc::clone(&token),
        Arc::clone(&intents),
        Arc::from(wecordy_api::constants::WS_URL),
    );
    websocket.connect().await?;

    Ok(())
}
