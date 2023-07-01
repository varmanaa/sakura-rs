use std::{collections::HashSet, sync::Arc};

use parking_lot::RwLock;
use time::OffsetDateTime;
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker, UserMarker},
    Id,
};

use crate::types::cache::{Cache, CurrentUser, CurrentUserUpdate};

impl Cache {
    pub fn get_current_user(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Option<Arc<CurrentUser>> {
        self.current_users
            .try_read()
            .map_or(None, |read_lock| read_lock.get(&guild_id).cloned())
    }

    pub fn insert_current_user(
        &self,
        guild_id: Id<GuildMarker>,
        communication_disabled_until: Option<OffsetDateTime>,
        user_id: Id<UserMarker>,
        role_ids: HashSet<Id<RoleMarker>>,
    ) {
        self.current_users.write().insert(
            guild_id,
            Arc::new(CurrentUser {
                communication_disabled_until,
                guild_id,
                user_id,
                role_ids: RwLock::new(role_ids),
            }),
        );
    }

    pub fn remove_current_user(
        &self,
        guild_id: Id<GuildMarker>,
    ) {
        self.current_users.write().remove(&guild_id);
    }

    pub fn update_current_user(
        &self,
        guild_id: Id<GuildMarker>,
        update: CurrentUserUpdate,
    ) {
        if let Some(old_current_user) = self.get_current_user(guild_id) {
            self.current_users.write().insert(
                guild_id,
                Arc::new(CurrentUser {
                    communication_disabled_until: update
                        .communication_disabled_until
                        .unwrap_or(old_current_user.communication_disabled_until),
                    guild_id,
                    user_id: old_current_user.user_id,
                    role_ids: RwLock::new(
                        update
                            .role_ids
                            .unwrap_or(old_current_user.role_ids.read().clone()),
                    ),
                }),
            );
        }
    }
}
