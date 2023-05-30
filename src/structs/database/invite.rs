use time::OffsetDateTime;
use tokio_postgres::types::ToSql;

use crate::types::{database::Database, Result};

impl Database {
    pub async fn get_unchecked_invites(
        &self,
        limit: u8,
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
                created_at ASC
            LIMIT $1;
        ";
        let params: &[&(dyn ToSql + Sync)] = &[&(limit as i8)];
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
