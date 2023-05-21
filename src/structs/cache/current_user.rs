use std::{collections::HashSet, sync::Arc};

use time::OffsetDateTime;
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};

use crate::types::cache::{Cache, CurrentUser};

impl Cache {
    pub fn get_current_user(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Option<Arc<CurrentUser>> {
        self.current_users.read().get(&guild_id).cloned()
    }

    pub fn insert_current_user(
        &self,
        guild_id: Id<GuildMarker>,
        communication_disabled_until: Option<OffsetDateTime>,
        role_ids: HashSet<Id<RoleMarker>>,
    ) {
        self.current_users.write().insert(
            guild_id,
            Arc::new(CurrentUser {
                communication_disabled_until,
                guild_id,
                role_ids,
            }),
        );
    }

    pub fn remove_current_user(
        &self,
        guild_id: Id<GuildMarker>,
    ) {
        self.current_users.write().remove(&guild_id);
    }
}
