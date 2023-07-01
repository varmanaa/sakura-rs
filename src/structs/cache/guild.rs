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

use crate::types::cache::{Cache, Guild, GuildUpdate};

impl Cache {
    pub fn get_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Option<Arc<Guild>> {
        self.guilds
            .try_read()
            .map_or(None, |read_lock| read_lock.get(&guild_id).cloned())
    }

    pub fn insert_guild(
        &self,
        channels: Vec<TwilightChannel>,
        guild_id: Id<GuildMarker>,
        in_check: bool,
        invite_check_category_ids: HashSet<Id<ChannelMarker>>,
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
                invite_check_category_ids: RwLock::new(invite_check_category_ids),
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

    pub fn update_guild(
        &self,
        guild_id: Id<GuildMarker>,
        update: GuildUpdate,
    ) {
        if let Some(old_guild) = self.get_guild(guild_id) {
            self.guilds.write().insert(
                guild_id,
                Arc::new(Guild {
                    channel_ids: RwLock::new(old_guild.channel_ids.read().clone()),
                    guild_id,
                    in_check: update.in_check.unwrap_or(old_guild.in_check),
                    invite_check_category_ids: RwLock::new(
                        update
                            .invite_check_category_ids
                            .unwrap_or(old_guild.invite_check_category_ids.read().clone()),
                    ),
                    name: update.name.unwrap_or(old_guild.name.clone()),
                    role_ids: RwLock::new(old_guild.role_ids.read().clone()),
                }),
            );
        }
    }
}
