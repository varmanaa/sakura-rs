use twilight_model::id::{marker::GuildMarker, Id};

use crate::types::cache::Cache;

impl Cache {
    pub fn insert_unavailable_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) {
        self.unavailable_guilds.write().insert(guild_id);
    }

    pub fn remove_unavailable_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) {
        self.unavailable_guilds.write().remove(&guild_id);
    }
}
