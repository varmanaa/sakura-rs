use serde::Serialize;
use tokio_postgres::types::ToSql;

use crate::{
    structs::Database,
    types::{
        database::{Event, GuildCreatePayload, GuildDeletePayload, InviteCheckCreatePayload},
        Result,
    },
};

impl Database {
    async fn insert_event(
        &self,
        event: Event,
        payload: impl Serialize,
    ) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            INSERT INTO
                public.event_log (event, payload)
            VALUES
                ($1, $2);
        ";

        let params: &[&(dyn ToSql + Sync)] = &[&event, &serde_json::to_value(payload)?];

        client.execute(statement, params).await?;

        Ok(())
    }

    pub async fn insert_guild_create_event(
        &self,
        payload: GuildCreatePayload,
    ) -> Result<()> {
        self.insert_event(Event::GuildCreate, payload).await?;

        Ok(())
    }

    pub async fn insert_guild_delete_event(
        &self,
        payload: GuildDeletePayload,
    ) -> Result<()> {
        self.insert_event(Event::GuildDelete, payload).await?;

        Ok(())
    }

    pub async fn insert_invite_check_create_event(
        &self,
        payload: InviteCheckCreatePayload,
    ) -> Result<()> {
        self.insert_event(Event::InviteCheckCreate, payload).await?;

        Ok(())
    }
}
