use std::sync::Arc;

use twilight_model::{
    guild::Role as TwilightRole,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

use crate::types::cache::{Cache, Role};

impl Cache {
    pub fn get_role(
        &self,
        role_id: Id<RoleMarker>,
    ) -> Option<Arc<Role>> {
        self.roles.read().get(&role_id).cloned()
    }

    pub fn insert_role(
        &self,
        guild_id: Id<GuildMarker>,
        role: TwilightRole,
    ) {
        let TwilightRole {
            id: role_id,
            permissions,
            ..
        } = role;

        if let Some(guild) = self.get_guild(guild_id) {
            guild.role_ids.write().insert(role_id);
        }

        self.roles.write().insert(
            role_id,
            Arc::new(Role {
                guild_id,
                permissions,
                role_id,
            }),
        );
    }

    pub fn remove_role(
        &self,
        role_id: Id<RoleMarker>,
    ) {
        if let Some(role) = self.roles.write().remove(&role_id) {
            if let Some(guild) = self.get_guild(role.guild_id) {
                guild.role_ids.write().remove(&role_id);
            }
        }
    }
}
