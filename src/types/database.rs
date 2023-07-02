use std::collections::HashSet;

use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::types::{FromSql, ToSql};
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

pub struct Database {
    pub pool: Pool,
}

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "event")]
pub enum Event {
    #[postgres(name = "GUILD_CREATE")]
    GuildCreate,
    #[postgres(name = "GUILD_DELETE")]
    GuildDelete,
    #[postgres(name = "INVITE_CHECK_CREATE")]
    InviteCheckCreate,
}

pub struct Guild {
    pub guild_id: Id<GuildMarker>,
    pub category_channel_ids: HashSet<Id<ChannelMarker>>,
    pub ignored_channel_ids: HashSet<Id<ChannelMarker>>,
    pub embed_color: i32,
    pub results_channel_id: Option<Id<ChannelMarker>>,
}

#[derive(Deserialize, Serialize)]
pub struct GuildCreatePayload {
    pub guild_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct GuildDeletePayload {
    pub guild_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct InviteCheckCreatePayload {
    pub guild_id: i64,
    pub start_time: OffsetDateTime,
    pub end_time: OffsetDateTime,
    pub channels: i64,
    pub valid_invites: i64,
    pub invalid_invites: i64,
    pub unknown_invites: i64,
}
