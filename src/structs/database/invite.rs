use time::OffsetDateTime;
use tokio_postgres::types::ToSql;

use crate::types::{database::Database, Result};

impl Database {
    pub async fn insert_invite(
        &self,
        code: &str,
        is_permalink: Option<bool>,
        is_valid: Option<bool>,
        expires_at: Option<OffsetDateTime>,
        updated_at: Option<OffsetDateTime>,
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
}
