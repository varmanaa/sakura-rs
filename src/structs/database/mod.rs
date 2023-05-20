mod event_log;
mod guild;
mod invite;
mod message;

use std::str::FromStr;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{Config, NoTls};

use crate::{types::Result, utility::DATABASE_URL};

pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn create_tables(&self) -> Result<()> {
        let client = self.pool.get().await?;

        let statement = "
            -- event enum
            DO $$
            BEGIN
                DROP TYPE IF EXISTS event;
                CREATE TYPE event AS ENUM (
                    'GUILD_CREATE',
                    'GUILD_DELETE',
                    'INVITE_CHECK_CREATE'
                );
            END $$;
            
            -- event_log table
            CREATE TABLE IF NOT EXISTS public.event_log (
                id BIGSERIAL PRIMARY KEY,
                event event NOT NULL,
                payload JSONB NOT NULL DEFAULT '{}'::JSONB,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            -- guild table
            CREATE TABLE IF NOT EXISTS public.guild (
                guild_id INT8 PRIMARY KEY,
                category_channel_ids INT8[] NOT NULL DEFAULT '{}',
                ignored_channel_ids INT8[] NOT NULL DEFAULT '{}',
                embed_color INT4 NOT NULL DEFAULT 16316671,
                results_channel_id INT8,
                last_checked_at TIMESTAMP WITH TIME ZONE
            );

            -- invite table
            CREATE TABLE IF NOT EXISTS public.invite (
                code TEXT PRIMARY KEY,
                is_permalink BOOLEAN DEFAULT NULL,
                is_valid BOOLEAN DEFAULT NULL,
                expires_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
                inserted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
            );

            -- message table
            CREATE TABLE IF NOT EXISTS public.message (
                guild_id INT8,
                channel_id INT8,
                message_id INT8,
                category_id INT8 NOT NULL,
                invite_codes TEXT[] NOT NULL DEFAULT '{}',
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (guild_id, channel_id, message_id)
            );
        ";

        client.batch_execute(statement).await?;

        Ok(())
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            pool: Pool::builder(Manager::from_config(
                Config::from_str(DATABASE_URL.as_str())?,
                NoTls,
                ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                },
            ))
            .max_size(16)
            .build()?,
        })
    }
}
