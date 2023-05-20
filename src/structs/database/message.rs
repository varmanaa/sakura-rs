use tokio_postgres::types::ToSql;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker},
    Id,
};

use crate::{structs::Database, types::Result};

impl Database {
    pub async fn insert_message(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        category_id: Id<ChannelMarker>,
        invite_codes: Vec<String>,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            INSERT INTO
                public.message
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
            &(category_id.get() as i64),
            &invite_codes,
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
                message_ids IN $1;
        ";

        let params: &[&(dyn ToSql + Sync)] = &[&message_ids
            .into_iter()
            .map(|message_id| message_id.get() as i64)
            .collect::<Vec<i64>>()];

        client.execute(statement, params).await?;

        Ok(())
    }
}
