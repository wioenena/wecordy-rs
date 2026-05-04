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
define_route!(DELETE, delete_channel_by_id, "/channel/{}", channel_id);
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

#[cfg(test)]
mod tests {
    use super::*;
    use http::Method;
    use std::borrow::Cow;

    macro_rules! create_test_route {
        ($method: ident, $path: literal) => {
            $crate::routes::RouteMeta {
                method: Method::$method,
                path: Cow::Owned($path.to_owned()),
            }
        };
    }

    #[test]
    fn test_user_routes() {
        assert_eq!(get_current_user(), create_test_route!(GET, "/user"));
        assert_eq!(get_user_by_id(1), create_test_route!(GET, "/user/1"));
        assert_eq!(update_current_user(), create_test_route!(PATCH, "/user"))
    }

    #[test]
    fn test_application_routes() {
        assert_eq!(
            get_current_application(),
            create_test_route!(GET, "/application/@me")
        );
        assert_eq!(get_applications(), create_test_route!(GET, "/application"));
        assert_eq!(
            create_application(),
            create_test_route!(POST, "/application")
        );
        assert_eq!(
            get_application_by_id(1),
            create_test_route!(GET, "/application/1")
        );
        assert_eq!(
            update_application_by_id(1),
            create_test_route!(PATCH, "/application/1")
        );
        assert_eq!(
            delete_application_by_id(1),
            create_test_route!(DELETE, "/application/1")
        );
        assert_eq!(
            get_application_servers(1),
            create_test_route!(GET, "/application/1/servers")
        )
    }

    #[test]
    fn test_application_command_routes() {
        assert_eq!(
            get_application_commands(1),
            create_test_route!(GET, "/application/1/commands")
        );
        assert_eq!(
            create_application_command(1),
            create_test_route!(POST, "/application/1/commands")
        );
        assert_eq!(
            update_application_commands(1),
            create_test_route!(PUT, "/application/1/commands")
        );

        assert_eq!(
            get_application_command_by_id(1, 2),
            create_test_route!(GET, "/application/1/commands/2")
        );
        assert_eq!(
            update_application_command_by_id(1, 2),
            create_test_route!(PATCH, "/application/1/commands/2")
        );
        assert_eq!(
            delete_application_command_by_id(1, 2),
            create_test_route!(DELETE, "/application/1/commands/2")
        );
        assert_eq!(
            get_application_server_commands(1, 2),
            create_test_route!(GET, "/application/1/servers/2/commands")
        );
        assert_eq!(
            create_application_server_commands(1, 2),
            create_test_route!(POST, "/application/1/servers/2/commands")
        );
        assert_eq!(
            update_application_server_commands(1, 2),
            create_test_route!(PUT, "/application/1/servers/2/commands")
        );
        assert_eq!(
            get_application_server_command(1, 2, 3),
            create_test_route!(GET, "/application/1/servers/2/commands/3")
        );
        assert_eq!(
            update_application_server_command(1, 2, 3),
            create_test_route!(PATCH, "/application/1/servers/2/commands/3")
        );
        assert_eq!(
            delete_application_server_command(1, 2, 3),
            create_test_route!(DELETE, "/application/1/servers/2/commands/3")
        );
    }

    #[test]
    fn test_server_routes() {
        assert_eq!(get_servers(), create_test_route!(GET, "/servers"));
        assert_eq!(get_server_by_id(1), create_test_route!(GET, "/servers/1"));
        assert_eq!(
            update_server_by_id(1),
            create_test_route!(PATCH, "/servers/1")
        );
        assert_eq!(
            get_server_members(1),
            create_test_route!(GET, "/server-has-user/1")
        );
        assert_eq!(
            get_server_member_by_id(1, 2),
            create_test_route!(GET, "/server-has-user/1/user/2")
        );
        assert_eq!(
            get_server_online_users(),
            create_test_route!(POST, "/server-has-user/online")
        );
    }

    #[test]
    fn test_channel_routes() {
        assert_eq!(
            get_server_channels(1),
            create_test_route!(GET, "/channel/1/channels")
        );
        assert_eq!(create_channel(), create_test_route!(POST, "/channel"));
        assert_eq!(get_channel_by_id(1), create_test_route!(GET, "/channel/1"));
        assert_eq!(
            update_channel_by_id(1),
            create_test_route!(PATCH, "/channel/1")
        );
        assert_eq!(
            delete_channel_by_id(1),
            create_test_route!(DELETE, "/channel/1")
        );
        assert_eq!(
            join_voice_channel(),
            create_test_route!(POST, "/channel/join")
        );
        assert_eq!(
            disconnect_voice_channel(),
            create_test_route!(POST, "/channel/disconnect")
        );
        assert_eq!(
            add_tracks_to_voice_channel(),
            create_test_route!(POST, "/channel/add-tracks")
        );
    }

    #[test]
    fn test_cloudflare_calls() {
        println!("TODO:");
    }

    #[test]
    fn test_message_routes() {
        assert_eq!(
            get_channel_messages(),
            create_test_route!(POST, "/message/channel")
        );
        assert_eq!(
            create_channel_message(),
            create_test_route!(PUT, "/message/channel")
        );
        assert_eq!(
            delete_channel_message(),
            create_test_route!(DELETE, "/message/channel")
        );
        assert_eq!(update_message(), create_test_route!(PATCH, "/message"));
        assert_eq!(
            toggle_channel_message_reaction(),
            create_test_route!(PUT, "/message/channel/reaction")
        );
        assert_eq!(
            get_channel_pinned_messages(1),
            create_test_route!(GET, "/channels/1/pins")
        );
        assert_eq!(
            pin_channel_message(1, 2),
            create_test_route!(POST, "/channels/1/pins/2")
        );
    }

    #[test]
    fn test_direct_message_routes() {
        assert_eq!(get_direct_messages(), create_test_route!(POST, "/message"));
        assert_eq!(send_direct_message(), create_test_route!(PUT, "/message"));
    }

    #[test]
    fn test_role_routes() {
        assert_eq!(
            get_server_roles(1),
            create_test_route!(GET, "/servers/1/roles")
        );
        assert_eq!(
            create_server_role(1),
            create_test_route!(POST, "/servers/1/roles")
        );
        assert_eq!(
            update_server_role(1, 2),
            create_test_route!(PATCH, "/servers/1/roles/2")
        );
        assert_eq!(
            delete_server_role(1, 2),
            create_test_route!(DELETE, "/servers/1/roles/2")
        );
        assert_eq!(
            add_server_role_to_member(1),
            create_test_route!(POST, "/servers/1/roles/assign")
        );
        assert_eq!(
            remove_server_role_from_member(1),
            create_test_route!(POST, "/servers/1/roles/remove")
        );
    }

    #[test]
    fn test_webhook_routes() {
        assert_eq!(
            get_channel_webhooks(1),
            create_test_route!(GET, "/channel/1/webhooks")
        );
        assert_eq!(
            create_channel_webhook(1),
            create_test_route!(POST, "/channel/1/webhooks")
        );
        assert_eq!(update_webhook(1), create_test_route!(PATCH, "/webhooks/1"));
        assert_eq!(delete_webhook(1), create_test_route!(DELETE, "/webhooks/1"));
        assert_eq!(
            execute_webhook(1, "<token>"),
            create_test_route!(POST, "/webhooks/1/<token>")
        );
    }

    #[test]
    fn test_interaction_routes() {
        assert_eq!(interactions(), create_test_route!(POST, "/interactions"));
        assert_eq!(
            interaction_callback(1, "<token>"),
            create_test_route!(POST, "/interactions/1/<token>/callback")
        );
        assert_eq!(
            update_interaction_response(1, "<token>"),
            create_test_route!(PATCH, "/webhooks/1/<token>/messages/@original")
        );
    }

    #[test]
    fn test_invite_routes() {
        assert_eq!(
            create_server_invite(1),
            create_test_route!(POST, "/servers/1/invites")
        );
        assert_eq!(
            get_server_invites(1),
            create_test_route!(GET, "/servers/1/invites")
        );
    }

    #[test]
    fn test_audit_log_routes() {
        assert_eq!(
            get_server_audit_logs(1),
            create_test_route!(GET, "/servers/1/audit-logs")
        );
    }
}
