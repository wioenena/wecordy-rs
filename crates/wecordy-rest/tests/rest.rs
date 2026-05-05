use serde_json::json;
use std::time::Duration;
use wecordy_api::{
    models::{ApiResponse, channel::Channel, message::Message, user::User},
    routes,
};
use wecordy_rest::{Client, ClientBuilder, Result};

fn create_rest_client() -> Client {
    let token = std::env::var("WECORDY_TOKEN").unwrap();

    ClientBuilder::new(token)
        .with_timeout(Duration::from_secs(3))
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_get() -> Result<()> {
    let rest = create_rest_client();

    let user: ApiResponse<User> = rest.get(routes::get_current_user().path()).await?;
    assert!(
        user.data()
            .unwrap()
            .full_name()
            .unwrap()
            .contains("wecordy"),
    );
    Ok(())
}

#[tokio::test]
async fn test_post() -> Result<()> {
    let rest = create_rest_client();
    let server_id = std::env::var("SERVER_ID").unwrap();
    let channel: ApiResponse<Channel> = rest
        .post::<serde_json::Value, Channel>(
            routes::create_channel().path(),
            Some(&json!({
                "server_id": server_id,
                "name": "channel-created-by-wecordy-rest",
                "type": "text"
            })),
        )
        .await?;

    assert_eq!(
        channel.data().unwrap().name(),
        "channel-created-by-wecordy-rest"
    );

    tokio::time::sleep(Duration::from_secs(1)).await;

    rest.delete::<serde_json::Value, ()>(
        routes::delete_channel().path(),
        Some(&json!({
            "server_id": server_id,
            "id": channel.data().unwrap().id()
        })),
    )
    .await?;

    Ok(())
}

#[tokio::test]
async fn test_put() -> Result<()> {
    let rest = create_rest_client();

    let channel_id = std::env::var("CHANNEL_ID").unwrap();
    let message: ApiResponse<Message> = rest
        .put(
            routes::create_channel_message().path(),
            Some(&json!({
                "channel_id": channel_id,
                "text": "message created by wecordy-rest"
            })),
        )
        .await?;

    assert_eq!(
        message.data().unwrap().text(),
        "message created by wecordy-rest"
    );

    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<()> {
    let rest = create_rest_client();

    let channel_id = std::env::var("CHANNEL_ID").unwrap();
    let message: ApiResponse<Message> = rest
        .put(
            routes::create_channel_message().path(),
            Some(&json!({
                "channel_id": channel_id,
                "text": "message created by wecordy-rest. It will be deleted in 2 seconds."
            })),
        )
        .await?;

    tokio::time::sleep(Duration::from_secs(2)).await;
    let response: ApiResponse<()> = rest
        .delete(
            routes::delete_channel_message().path(),
            Some(&json!({
                "channel_id":channel_id,
                "message_id": message.data().unwrap().id(),
            })),
        )
        .await?;

    assert_eq!(response.status_code(), 200);
    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<()> {
    let rest = create_rest_client();

    let channel_id = std::env::var("CHANNEL_ID").unwrap();
    let message: ApiResponse<Message> = rest
        .put(
            routes::create_channel_message().path(),
            Some(&json!({
                "channel_id":channel_id,
                "text": "message created by wecordy-rest. It will be updated in 2 seconds."
            })),
        )
        .await?;

    assert!(message.data().is_some());
    tokio::time::sleep(Duration::from_secs(2)).await;
    let updated_message_response: ApiResponse<Message> = rest
        .patch(
            routes::update_message().path(),
            Some(&json!({
                "message_id": message.data().unwrap().id(),
                "text": "message updated by wecordy-rest."
            })),
        )
        .await?;

    assert_eq!(updated_message_response.status_code(), 200);
    Ok(())
}
