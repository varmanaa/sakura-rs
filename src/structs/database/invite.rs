use std::collections::HashMap;

use time::OffsetDateTime;
use tokio_postgres::types::ToSql;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

use crate::types::{database::Database, Result};

impl Database {
    pub async fn get_guild_invite_counts(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> Result<HashMap<Id<ChannelMarker>, (u16, u16, u16)>> {
        let client = self.pool.get().await?;

        let statement = "
            WITH guild_invite AS (
                SELECT
                    public.message.channel_id,
                    _.code,
                    COALESCE(public.invite.is_valid, FALSE) AS is_valid,
                    CASE
                        WHEN public.invite.updated_at IS NULL THEN FALSE
                        ELSE TRUE
                    END AS is_updated
                FROM
                    public.message,
                    UNNEST(public.message.invite_codes) _(code)
                    LEFT JOIN public.invite ON public.invite.code = _.code
                WHERE
                    guild_id = $1
            )
            SELECT
                channel_id,
                COUNT(*) FILTER (WHERE guild_invite.is_valid AND guild_invite.is_updated)::INT2 AS valid_invites,
                COUNT(*) FILTER (WHERE NOT guild_invite.is_valid AND guild_invite.is_updated)::INT2 AS invalid_invites,
                COUNT(*) FILTER (WHERE NOT guild_invite.is_updated)::INT2 AS unknown_invites
            FROM
                guild_invite
            GROUP BY
                guild_invite.channel_id;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(guild_id.get() as i64)];
        let mut invite_check: HashMap<Id<ChannelMarker>, (u16, u16, u16)> = HashMap::new();

        if let Ok(rows) = client.query(statement, params).await {
            for row in rows {
                invite_check.insert(
                    Id::new(row.get::<_, i64>("channel_id") as u64),
                    (
                        row.get::<_, i16>("valid_invites") as u16,
                        row.get::<_, i16>("invalid_invites") as u16,
                        row.get::<_, i16>("unknown_invites") as u16,
                    ),
                );
            }
        }

        Ok(invite_check)
    }

    pub async fn get_unchecked_invites(
        &self,
        limit: u64,
    ) -> Result<Vec<String>> {
        let client = self.pool.get().await?;

        let statement = "
            SELECT
                code
            FROM
                public.invite
            WHERE
                updated_at IS NULL
            ORDER BY
                created_at
            LIMIT $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(limit as i64)];
        let rows = client.query(statement, params).await?;
        let codes = rows
            .into_iter()
            .map(|row| row.get("code"))
            .collect::<Vec<String>>();

        Ok(codes)
    }

    pub async fn insert_unchecked_invite(
        &self,
        code: &str,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            INSERT INTO
                public.invite (code)
            VALUES
                ($1)
            ON CONFLICT (code)
            DO NOTHING;
        ";

        let params: &[&(dyn ToSql + Sync)] = &[&code];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn insert_checked_invite(
        &self,
        code: &str,
        is_permalink: bool,
        is_valid: bool,
        expires_at: Option<OffsetDateTime>,
        updated_at: OffsetDateTime,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            INSERT INTO
                public.invite (
                    code,
                    is_permalink,
                    is_valid,
                    expires_at,
                    updated_at
                )
            VALUES
                ($1, $2, $3, $4, $5)
            ON CONFLICT (code)
            DO UPDATE
            SET
                is_permalink = EXCLUDED.is_permalink,
                is_valid = EXCLUDED.is_valid,
                expires_at = EXCLUDED.expires_at,
                updated_at = EXCLUDED.updated_at;
        ";

        let params: &[&(dyn ToSql + Sync)] =
            &[&code, &is_permalink, &is_valid, &expires_at, &updated_at];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn remove_old_invites(
        &self,
        age: u8,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            DELETE FROM
                public.invite
            WHERE
                created_at >= CURRENT_TIMESTAMP - INTERVAL '$1 days';
        ";

        let params: &[&(dyn ToSql + Sync)] = &[&(age as i8)];

        client.execute(statement, params).await?;

        Ok(())
    }
}
