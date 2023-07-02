use std::{
    collections::{HashMap, HashSet},
    iter,
};

use tokio_postgres::types::ToSql;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};

use crate::types::{database::Database, Result};

impl Database {
    pub async fn insert_message(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        parent_id: Id<ChannelMarker>,
        invite_codes: HashSet<String>,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            INSERT INTO
                public.message (guild_id, channel_id, message_id, parent_id, invite_codes)
            VALUES
                ($1, $2, $3, $4, $5)
            ON CONFLICT (guild_id, channel_id, message_id)
            DO UPDATE
            SET
                invite_codes = EXCLUDED.invite_codes;
        ";

        let params: &[&(dyn ToSql + Sync)] = &[
            &(guild_id.get() as i64),
            &(channel_id.get() as i64),
            &(message_id.get() as i64),
            &(parent_id.get() as i64),
            &Vec::from_iter(invite_codes),
        ];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_channel_messages(
        &self,
        channel_id: Id<ChannelMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            DELETE FROM
                public.message
            WHERE
                channel_id = $1;
        ";

        let params: &[&(dyn ToSql + Sync)] = &[&(channel_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_guild_messages(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            DELETE FROM
                public.message
            WHERE
                guild_id = $1;
        ";

        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_messages(
        &self,
        message_ids: Vec<Id<MessageMarker>>,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            DELETE FROM
                public.message
            WHERE
                message_id = ANY($1);
        ";

        let params: &[&(dyn ToSql + Sync)] = &[&message_ids
            .into_iter()
            .map(|message_id| message_id.get() as i64)
            .collect::<Vec<i64>>()];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_old_messages(
        &self
    ) -> Result<HashMap<Id<GuildMarker>, HashSet<Id<ChannelMarker>>>> {
        let client = self.pool.get().await?;

        let statement = "
            DELETE FROM
                public.message
            WHERE
                created_at >= CURRENT_TIMESTAMP - INTERVAL '14 days'
            RETURNING
                guild_id,
                channel_id;
        ";

        let params: &[&(dyn ToSql + Sync)] = &[];
        let mut removed_ids: HashMap<Id<GuildMarker>, HashSet<Id<ChannelMarker>>> = HashMap::new();

        if let Ok(rows) = client.query(statement, params).await {
            for row in rows {
                let guild_id: Id<GuildMarker> = Id::new(row.get::<_, i64>("guild_id") as u64);
                let channel_id: Id<ChannelMarker> = Id::new(row.get::<_, i64>("channel_id") as u64);

                if let Some(channel_ids) = removed_ids.get_mut(&guild_id) {
                    channel_ids.insert(channel_id);
                } else {
                    removed_ids.insert(guild_id, HashSet::from_iter(iter::once(channel_id)));
                }
            }
        }

        Ok(removed_ids)
    }
}
