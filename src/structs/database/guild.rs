use time::OffsetDateTime;
use tokio_postgres::{types::ToSql, Row};
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

use crate::types::{
    database::{Database, Guild},
    Result,
};

impl Database {
    pub async fn get_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Option<Guild> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            SELECT
                *
            FROM
                public.guild
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];

        client
            .query_one(statement, params)
            .await
            .map_or(None, |row| Some(row.into()))
    }

    pub async fn insert_category_channel(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            UPDATE
                public.guild
            SET
                category_channel_ids = ARRAY(SELECT DISTINCT UNNEST(ARRAY_APPEND(category_channel_ids, $2)))
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] =
            &[&(guild_id.get() as i64), &(channel_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn insert_embed_color(
        &self,
        guild_id: Id<GuildMarker>,
        embed_color: i32,
    ) -> Result<()> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            UPDATE
                public.guild
            SET
                embed_color = $2
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64), &embed_color];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn insert_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            INSERT INTO
                public.guild (guild_id)
            VALUES
                ($1)
            ON CONFLICT DO NOTHING;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn insert_ignored_channel(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            UPDATE
                public.guild
            SET
                ignored_channel_ids = ARRAY(SELECT DISTINCT UNNEST(ARRAY_APPEND(ignored_channel_ids, $2)))
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] =
            &[&(guild_id.get() as i64), &(channel_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn insert_last_checked_at(
        &self,
        guild_id: Id<GuildMarker>,
        last_checked_at: OffsetDateTime,
    ) -> Result<()> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            UPDATE
                public.guild
            SET
                last_checked_at = $2
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64), &last_checked_at];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn insert_results_channel(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            UPDATE
                public.guild
            SET
                results_channel_id = $2
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] =
            &[&(guild_id.get() as i64), &(channel_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_category_channel(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            UPDATE
                public.guild
            SET
                category_channel_ids = ARRAY(SELECT DISTINCT UNNEST(ARRAY_REMOVE(category_channel_ids, $2)))
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] =
            &[&(guild_id.get() as i64), &(channel_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await?;
        let statement = "
            DELETE FROM
                public.guild
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_ignored_channel(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            UPDATE
                public.guild
            SET
                ignored_channel_ids = ARRAY(SELECT DISTINCT UNNEST(ARRAY_REMOVE(ignored_channel_ids, $2)))
            WHERE
                guild_id = $1;
        ";
        let params: &[&(dyn ToSql + Sync)] =
            &[&(guild_id.get() as i64), &(channel_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_results_channel(
        &self,
        guild_id: Id<GuildMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Result<()> {
        let client = self.pool.get().await.unwrap();
        let statement = "
            UPDATE
                public.guild
            SET
                results_channel_id = NULL
            WHERE
                guild_id = $1
                AND channel_id = $2;
        ";

        let params: &[&(dyn ToSql + Sync)] =
            &[&(guild_id.get() as i64), &(channel_id.get() as i64)];

        client.execute(statement, params).await?;

        Ok(())
    }
}

impl From<Row> for Guild {
    fn from(row: Row) -> Self {
        Self {
            guild_id: Id::new(row.get::<_, i64>("guild_id") as u64),
            category_channel_ids: row
                .get::<_, Vec<i64>>("category_channel_ids")
                .into_iter()
                .map(|id| Id::new(id as u64))
                .collect(),
            ignored_channel_ids: row
                .get::<_, Vec<i64>>("ignored_channel_ids")
                .into_iter()
                .map(|id| Id::new(id as u64))
                .collect(),
            embed_color: row.get::<_, i32>("embed_color"),
            results_channel_id: row
                .try_get::<_, i64>("results_channel_id")
                .map_or(None, |channel_id| Some(Id::new(channel_id as u64))),
            last_checked_at: row.try_get::<_, OffsetDateTime>("last_checked_at").ok(),
        }
    }
}
