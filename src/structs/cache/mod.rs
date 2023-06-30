mod channel;
mod current_user;
mod guild;
mod role;
mod unavailable_guild;

use std::collections::{HashMap, HashSet};

use parking_lot::RwLock;
use twilight_model::{
    guild::Permissions,
    id::{
        marker::{ChannelMarker, RoleMarker},
        Id,
    },
};
use twilight_util::permission_calculator::PermissionCalculator;

use crate::types::cache::Cache;

impl Cache {
    pub fn has_minimum_channel_permissions(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> bool {
        let (guild_id, kind, permission_overwrites) = match self.get_channel(channel_id) {
            Some(channel) => {
                (
                    channel.guild_id,
                    channel.kind,
                    channel.permission_overwrites.clone().unwrap_or_default(),
                )
            }
            None => return false,
        };
        let current_user = match self.get_current_user(guild_id) {
            Some(current_user) => current_user,
            None => return false,
        };

        if current_user.communication_disabled_until.is_some() {
            return false;
        }

        let everyone_role_permissions = match self.get_role(guild_id.cast()) {
            Some(role) => role.permissions,
            None => return false,
        };
        let current_user_roles_and_permissions = current_user
            .role_ids
            .clone()
            .into_iter()
            .map(|role_id| {
                let permissions = self
                    .get_role(role_id)
                    .map_or(Permissions::from_bits_truncate(0), |role| role.permissions);

                (role_id, permissions)
            })
            .collect::<Vec<(Id<RoleMarker>, Permissions)>>();
        let calculator = PermissionCalculator::new(
            guild_id,
            current_user.user_id,
            everyone_role_permissions,
            &current_user_roles_and_permissions,
        );

        calculator
            .in_channel(kind, &permission_overwrites)
            .contains(
                Permissions::EMBED_LINKS
                    | Permissions::READ_MESSAGE_HISTORY
                    | Permissions::SEND_MESSAGES
                    | Permissions::VIEW_CHANNEL,
            )
    }

    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            current_users: RwLock::new(HashMap::new()),
            guilds: RwLock::new(HashMap::new()),
            roles: RwLock::new(HashMap::new()),
            unavailable_guilds: RwLock::new(HashSet::new()),
        }
    }
}
