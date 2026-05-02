use std::borrow::Cow;

#[derive(Debug)]
pub struct RouteMeta {
    method: reqwest::Method,
    path: Cow<'static, str>,
}

impl RouteMeta {
    #[inline]
    pub fn method(&self) -> &reqwest::Method {
        &self.method
    }

    #[inline]
    pub fn path(&self) -> &str {
        &self.path
    }
}

macro_rules! define_route_fn {
    ($method: ident, $name: ident, $path: literal) => {
        impl RouteMeta {
            #[inline]
            pub fn $name() -> RouteMeta {
                RouteMeta {
                    method: reqwest::Method::$method,
                    path: Cow::Borrowed($path),
                }
            }
        }
    };

    ($method: ident, $name: ident, $path: literal, $($arg: ident),+) => {
        impl RouteMeta {
            #[inline]
            pub fn $name($($arg: impl std::fmt::Display),+) -> RouteMeta {
                RouteMeta {
                    method: reqwest::Method::$method,
                    path: Cow::Owned(format!($path, $($arg),+)),
                }
            }
        }
    };
}

// User routes
define_route_fn!(GET, get_current_user, "/user");
define_route_fn!(GET, get_user_by_id, "/user/{}", user_id);
define_route_fn!(PATCH, update_current_user, "/user");

// Application routes
define_route_fn!(GET, get_current_application, "/application/@me");
define_route_fn!(GET, get_applications, "/application");
define_route_fn!(POST, create_application, "/application");
define_route_fn!(
    GET,
    get_application_by_id,
    "/application/{}",
    application_id
);
define_route_fn!(
    PATCH,
    update_application_by_id,
    "/application/{}",
    application_id
);
define_route_fn!(
    DELETE,
    delete_application_by_id,
    "/application/{}",
    application_id
);
define_route_fn!(
    GET,
    get_application_servers,
    "/application/{}/servers",
    application_id
);

// Application commands
define_route_fn!(
    GET,
    get_application_commands,
    "/application/{}/commands",
    application_id
);
define_route_fn!(
    POST,
    create_application_commands,
    "/application/{}/commands",
    application_id
);
define_route_fn!(
    PUT,
    update_application_commands,
    "/application/{}/commands",
    application_id
);
define_route_fn!(
    GET,
    get_application_command_by_id,
    "/application/{}/commands/{}",
    application_id,
    command_id
);
define_route_fn!(
    PATCH,
    update_application_command_by_id,
    "/application/{}/commands/{}",
    application_id,
    command_id
);
define_route_fn!(
    DELETE,
    delete_application_command_by_id,
    "/application/{}/commands/{}",
    application_id,
    command_id
);
define_route_fn!(
    GET,
    get_application_server_commands,
    "/application/{}/servers/{}/commands",
    application_id,
    server_id
);
define_route_fn!(
    POST,
    create_application_server_commands,
    "/application/{}/servers/{}/commands",
    application_id,
    server_id
);
define_route_fn!(
    PUT,
    update_application_server_commands,
    "/application/{}/servers/{}/commands",
    application_id,
    server_id
);
define_route_fn!(
    GET,
    get_application_server_command,
    "/application/{}/servers/{}/commands/{}",
    application_id,
    server_id,
    command_id
);

// Servers
define_route_fn!(GET, get_servers, "/servers");
define_route_fn!(GET, get_server_by_id, "/servers/{}", server_id);
define_route_fn!(PATCH, update_server_by_id, "/servers/{}", server_id);
define_route_fn!(GET, get_server_members, "/server-has-user/{}", server_id);
define_route_fn!(
    GET,
    get_server_member_by_id,
    "/server-has-user/{}/user/{}",
    server_id,
    user_id
);
define_route_fn!(POST, get_server_online_users, "/server-has-user/online");

// Channels
define_route_fn!(GET, get_server_channels, "/channel/{}/channels", server_id);
define_route_fn!(POST, create_channel, "/channel");
define_route_fn!(GET, get_channel_by_id, "/channel/{}", channel_id);
define_route_fn!(PATCH, update_channel_by_id, "/channel/{}", channel_id);
define_route_fn!(DELETE, delete_channel_by_id, "/channel/{}", channel_id);
define_route_fn!(POST, join_voice_channel, "/channel/join");
define_route_fn!(POST, disconnect_voice_channel, "/channel/disconnect");
define_route_fn!(POST, add_tracks_to_voice_channel, "/channel/add-tracks");

// TODO: implement cloudflare calls, what is this?

// Messages
define_route_fn!(POST, get_channel_messages, "/message/channel");
define_route_fn!(PUT, create_channel_message, "/message/channel");
define_route_fn!(DELETE, delete_channel_message, "/message/channel");
define_route_fn!(PATCH, update_message, "/message");
define_route_fn!(
    PUT,
    toggle_channel_message_reaction,
    "/message/channel/reaction"
);
define_route_fn!(
    GET,
    get_channel_pinned_messages,
    "/channels/{}/pins",
    channel_id
);
define_route_fn!(
    POST,
    pin_channel_message,
    "/channels/{}/pins/{}",
    channel_id,
    message_id
);

// DM messages
define_route_fn!(POST, get_direct_messages, "/message");
define_route_fn!(PUT, send_direct_message, "/message");

// Roles
define_route_fn!(GET, get_server_roles, "/servers/{}/roles", server_id);
define_route_fn!(POST, create_server_role, "/servers/{}/roles", server_id);
define_route_fn!(
    PATCH,
    update_server_role,
    "/servers/{}/roles/{}",
    server_id,
    role_id
);
define_route_fn!(
    DELETE,
    delete_server_role,
    "/servers/{}/roles/{}",
    server_id,
    role_id
);
define_route_fn!(
    POST,
    add_server_role_to_member,
    "/servers/{}/roles/assign",
    server_id
);
define_route_fn!(
    POST,
    remove_server_role_from_member,
    "/servers/{}/roles/remove",
    server_id
);

// Webhooks
define_route_fn!(
    GET,
    get_channel_webhooks,
    "/channel/{}/webhooks",
    channel_id
);
define_route_fn!(
    POST,
    create_channel_webhook,
    "/channel/{}/webhooks",
    channel_id
);
define_route_fn!(PATCH, update_webhook, "/webhooks/{}", webhook_id);
define_route_fn!(DELETE, delete_webhook, "/webhooks/{}", webhook_id);
define_route_fn!(
    POST,
    execute_webhook,
    "/webhooks/{}/{}",
    webhook_id,
    webhook_token
);

// Interactions
define_route_fn!(POST, interactions, "/interactions");
define_route_fn!(
    POST,
    interaction_callback,
    "/interactions/{}/{}/callback",
    interaction_id,
    interaction_token
);
define_route_fn!(
    PATCH,
    update_interaction_response,
    "/webhooks/{}/{}/messages/@original",
    application_id,
    interaction_token
);

// Invites
define_route_fn!(POST, create_server_invite, "/servers/{}/invites", server_id);
define_route_fn!(GET, get_server_invites, "/servers/{}/invites", server_id);

// Audit Log
define_route_fn!(
    GET,
    get_server_audit_logs,
    "/servers/{}/audit-logs",
    server_id
);
