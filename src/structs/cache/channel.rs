use core::matches;
use std::sync::Arc;

use twilight_model::{
    channel::{Channel as TwilightChannel, ChannelType},
    id::{marker::ChannelMarker, Id},
};

use crate::types::cache::{Cache, Channel, ChannelUpdate};

impl Cache {
    pub fn get_channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Option<Arc<Channel>> {
        self.channels
            .try_read()
            .map_or(None, |read_lock| read_lock.get(&channel_id).cloned())
    }

    pub fn insert_channel(
        &self,
        channel: TwilightChannel,
    ) {
        if let Some(guild_id) = channel.guild_id {
            if matches!(
                channel.kind,
                ChannelType::GuildAnnouncement
                    | ChannelType::GuildCategory
                    | ChannelType::GuildText
            ) {
                let channel_id = channel.id;

                if let Some(guild) = self.get_guild(guild_id) {
                    guild.channel_ids.write().insert(channel_id);
                }

                self.channels.write().insert(
                    channel_id,
                    Arc::new(Channel {
                        channel_id,
                        guild_id,
                        kind: channel.kind,
                        name: channel.name.unwrap_or_default(),
                        parent_id: channel.parent_id,
                        permission_overwrites: channel.permission_overwrites,
                        position: channel.position.unwrap_or_default(),
                    }),
                );
            }
        }
    }

    pub fn remove_channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) {
        if let Some(channel) = self.channels.write().remove(&channel_id) {
            if let Some(guild) = self.get_guild(channel.guild_id) {
                guild.channel_ids.write().remove(&channel_id);
                guild.invite_check_category_ids.write().remove(&channel_id);
            }
        }
    }

    pub fn update_channel(
        &self,
        channel_id: Id<ChannelMarker>,
        update: ChannelUpdate,
    ) {
        if let Some(old_channel) = self.get_channel(channel_id) {
            self.channels.write().insert(
                channel_id,
                Arc::new(Channel {
                    channel_id,
                    guild_id: old_channel.guild_id,
                    kind: update.kind.unwrap_or(old_channel.kind),
                    name: update.name.unwrap_or(old_channel.name.clone()),
                    parent_id: update.parent_id.unwrap_or(old_channel.parent_id),
                    permission_overwrites: update
                        .permission_overwrites
                        .unwrap_or(old_channel.permission_overwrites.clone()),
                    position: update.position.unwrap_or(old_channel.position),
                }),
            );
        }
    }
}
