use serde::Deserialize;
use serde_json::{Value, value::RawValue};

#[derive(Debug)]
pub enum GatewayReceivePayload {
    Me(Box<wecordy_api::models::user::User>),
    NewMessage,
    NewChannelMessage,
    DeleteMessage,
    DeleteChannelMessage,
    EditMessage,
    MessageReactionAdd,
    MessageReactionRemove,
    ChannelMessageReactionAdd,
    ChannelMessageReactionRemove,
    NewChannel,
    UpdateChannel,
    UpdateChannelPositions,
    DeleteChannel,
    NewChannelEvent,
    NewUser,
    UpdateUser,
    DeleteUser,
    NewServerUser,
    UpdateServerUser,
    DeleteServerUser,
    NewServer,
    UpdateServer,
    DeleteServer,
    UserJoinChannel,
    UserDisconnectChannel,
    NewUserInvite,
    UserInviteAccepted,
    UserBlocked,
    UserInviteRejected,
    UserUnblocked,
    UserRemoveFriend,
    WritingMessageUser,
    WritingMessageChannel,
    StopWritingMessageUser,
    StopWritingMessageChannel,
    MarkMessagesAsRead,
    UnreadMessage,
    NewRole,
    UpdateRole,
    DeleteRole,
    RoleAssigned,
    RoleRemoved,
    PinMessage,
    UnpinMessage,
    PermissionUpdate,
    UserBanned,
    UserUnbanned,
    MeBannedFromServer,
    UserKicked,
    MeKickedFromServer,
    UserMovedChannel,
    NewForumPost,
    UpdateForumPost,
    DeleteForumPost,
    ForumPostReactionAdd,
    ForumPostReactionRemove,
    InteractionCreate,
    ApplicationCommandCreate,
    ApplicationCommandUpdate,
    ApplicationCommandDelete,
}

#[derive(Deserialize)]
struct RawPayload {
    #[serde(rename = "type")]
    event_type: String,
    data: Value,
}

impl<'de> Deserialize<'de> for GatewayReceivePayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawPayload::deserialize(deserializer)?;

        match raw.event_type.as_str() {
            "ME" => todo!(),
            _ => Err(serde::de::Error::custom(format!(
                "unknowen event: {}",
                raw.event_type
            ))),
        }
    }
}
