use core::matches;
use std::sync::Arc;

use twilight_model::{
    channel::{Channel as TwilightChannel, ChannelType},
    id::{marker::ChannelMarker, Id},
};

use crate::types::cache::{Cache, Channel};

impl Cache {
    pub fn get_channel(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Option<Arc<Channel>> {
        self.channels.read().get(&channel_id).cloned()
    }

    pub fn insert_channel(
        &self,
        channel: TwilightChannel,
    ) {
        if let (Some(guild_id), Some(position)) = (channel.guild_id, channel.position) {
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
                        parent_id: channel.parent_id,
                        permission_overwrites: channel.permission_overwrites,
                        position,
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
}
