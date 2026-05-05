mod macros;

mod meta;

use crate::define_route;
pub use meta::RouteMeta;

// User routes
define_route!(GET, get_current_user, "/user");
define_route!(GET, get_user_by_id, "/user/{}", user_id);
define_route!(PATCH, update_current_user, "/user");

// Application routes
define_route!(GET, get_current_application, "/application/@me");
define_route!(GET, get_applications, "/application");
define_route!(POST, create_application, "/application");
define_route!(
    GET,
    get_application_by_id,
    "/application/{}",
    application_id
);
define_route!(
    PATCH,
    update_application_by_id,
    "/application/{}",
    application_id
);
define_route!(
    DELETE,
    delete_application_by_id,
    "/application/{}",
    application_id
);
define_route!(
    GET,
    get_application_servers,
    "/application/{}/servers",
    application_id
);

// Application commands
define_route!(
    GET,
    get_application_commands,
    "/application/{}/commands",
    application_id
);
define_route!(
    POST,
    create_application_command,
    "/application/{}/commands",
    application_id
);
define_route!(
    PUT,
    update_application_commands,
    "/application/{}/commands",
    application_id
);
define_route!(
    GET,
    get_application_command_by_id,
    "/application/{}/commands/{}",
    application_id,
    command_id
);
define_route!(
    PATCH,
    update_application_command_by_id,
    "/application/{}/commands/{}",
    application_id,
    command_id
);
define_route!(
    DELETE,
    delete_application_command_by_id,
    "/application/{}/commands/{}",
    application_id,
    command_id
);
define_route!(
    GET,
    get_application_server_commands,
    "/application/{}/servers/{}/commands",
    application_id,
    server_id
);
define_route!(
    POST,
    create_application_server_commands,
    "/application/{}/servers/{}/commands",
    application_id,
    server_id
);
define_route!(
    PUT,
    update_application_server_commands,
    "/application/{}/servers/{}/commands",
    application_id,
    server_id
);
define_route!(
    GET,
    get_application_server_command,
    "/application/{}/servers/{}/commands/{}",
    application_id,
    server_id,
    command_id
);
define_route!(
    PATCH,
    update_application_server_command,
    "/application/{}/servers/{}/commands/{}",
    application_id,
    server_id,
    command_id
);
define_route!(
    DELETE,
    delete_application_server_command,
    "/application/{}/servers/{}/commands/{}",
    application_id,
    server_id,
    command_id
);

// Servers
define_route!(GET, get_servers, "/servers");
define_route!(GET, get_server_by_id, "/servers/{}", server_id);
define_route!(PATCH, update_server_by_id, "/servers/{}", server_id);
define_route!(GET, get_server_members, "/server-has-user/{}", server_id);
define_route!(
    GET,
    get_server_member_by_id,
    "/server-has-user/{}/user/{}",
    server_id,
    user_id
);
define_route!(POST, get_server_online_users, "/server-has-user/online");

// Channels
define_route!(GET, get_server_channels, "/channel/{}/channels", server_id);
define_route!(POST, create_channel, "/channel");
define_route!(GET, get_channel_by_id, "/channel/{}", channel_id);
define_route!(PATCH, update_channel_by_id, "/channel/{}", channel_id);
define_route!(DELETE, delete_channel, "/channel");
define_route!(POST, join_voice_channel, "/channel/join");
define_route!(POST, disconnect_voice_channel, "/channel/disconnect");
define_route!(POST, add_tracks_to_voice_channel, "/channel/add-tracks");

// TODO: implement cloudflare calls, what is this?

// Messages
define_route!(POST, get_channel_messages, "/message/channel");
define_route!(PUT, create_channel_message, "/message/channel");
define_route!(DELETE, delete_channel_message, "/message/channel");
define_route!(PATCH, update_message, "/message");
define_route!(
    PUT,
    toggle_channel_message_reaction,
    "/message/channel/reaction"
);
define_route!(
    GET,
    get_channel_pinned_messages,
    "/channels/{}/pins",
    channel_id
);
define_route!(
    POST,
    pin_channel_message,
    "/channels/{}/pins/{}",
    channel_id,
    message_id
);

// DM messages
define_route!(POST, get_direct_messages, "/message");
define_route!(PUT, send_direct_message, "/message");

// Roles
define_route!(GET, get_server_roles, "/servers/{}/roles", server_id);
define_route!(POST, create_server_role, "/servers/{}/roles", server_id);
define_route!(
    PATCH,
    update_server_role,
    "/servers/{}/roles/{}",
    server_id,
    role_id
);
define_route!(
    DELETE,
    delete_server_role,
    "/servers/{}/roles/{}",
    server_id,
    role_id
);
define_route!(
    POST,
    add_server_role_to_member,
    "/servers/{}/roles/assign",
    server_id
);
define_route!(
    POST,
    remove_server_role_from_member,
    "/servers/{}/roles/remove",
    server_id
);

// Webhooks
define_route!(
    GET,
    get_channel_webhooks,
    "/channel/{}/webhooks",
    channel_id
);
define_route!(
    POST,
    create_channel_webhook,
    "/channel/{}/webhooks",
    channel_id
);
define_route!(PATCH, update_webhook, "/webhooks/{}", webhook_id);
define_route!(DELETE, delete_webhook, "/webhooks/{}", webhook_id);
define_route!(
    POST,
    execute_webhook,
    "/webhooks/{}/{}",
    webhook_id,
    webhook_token
);

// Interactions
define_route!(POST, interactions, "/interactions");
define_route!(
    POST,
    interaction_callback,
    "/interactions/{}/{}/callback",
    interaction_id,
    interaction_token
);
define_route!(
    PATCH,
    update_interaction_response,
    "/webhooks/{}/{}/messages/@original",
    application_id,
    interaction_token
);

// Invites
define_route!(POST, create_server_invite, "/servers/{}/invites", server_id);
define_route!(GET, get_server_invites, "/servers/{}/invites", server_id);

// Audit Log
define_route!(
    GET,
    get_server_audit_logs,
    "/servers/{}/audit-logs",
    server_id
);
