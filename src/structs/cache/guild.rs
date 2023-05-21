use std::{collections::HashSet, sync::Arc};

use parking_lot::RwLock;
use twilight_model::{
    channel::Channel as TwilightChannel,
    guild::Role as TwilightRole,
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker},
        Id,
    },
};

use crate::types::cache::{Cache, Guild};

impl Cache {
    pub fn get_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Option<Arc<Guild>> {
        self.guilds.read().get(&guild_id).cloned()
    }

    pub fn insert_guild(
        &self,
        channels: Vec<TwilightChannel>,
        guild_id: Id<GuildMarker>,
        in_check: bool,
        name: String,
        roles: Vec<TwilightRole>,
    ) {
        let mut channel_ids: HashSet<Id<ChannelMarker>> = HashSet::new();
        let mut role_ids: HashSet<Id<RoleMarker>> = HashSet::new();

        for channel in channels {
            channel_ids.insert(channel.id);
            self.insert_channel(channel);
        }

        for role in roles {
            role_ids.insert(role.id);
            self.insert_role(guild_id, role);
        }

        self.guilds.write().insert(
            guild_id,
            Arc::new(Guild {
                channel_ids: RwLock::new(channel_ids),
                guild_id,
                in_check,
                name,
                role_ids: RwLock::new(role_ids),
            }),
        );
        self.remove_unavailable_guild(guild_id)
    }

    pub fn remove_guild(
        &self,
        guild_id: Id<GuildMarker>,
        unavailable: bool,
    ) {
        if let Some(guild) = self.guilds.write().remove(&guild_id) {
            for channel_id in guild.channel_ids.read().iter() {
                self.remove_channel(*channel_id);
            }

            self.remove_current_user(guild_id);

            for role_id in guild.role_ids.read().iter() {
                self.remove_role(*role_id);
            }
        }
        if unavailable {
            self.insert_unavailable_guild(guild_id)
        }
    }
}
