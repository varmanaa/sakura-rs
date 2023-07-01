use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use parking_lot::RwLock;
use time::OffsetDateTime;
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    guild::Permissions,
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};

pub struct Cache {
    pub channels: RwLock<HashMap<Id<ChannelMarker>, Arc<Channel>>>,
    pub current_users: RwLock<HashMap<Id<GuildMarker>, Arc<CurrentUser>>>,
    pub guilds: RwLock<HashMap<Id<GuildMarker>, Arc<Guild>>>,
    pub roles: RwLock<HashMap<Id<RoleMarker>, Arc<Role>>>,
    pub unavailable_guilds: RwLock<HashSet<Id<GuildMarker>>>,
}

#[derive(Debug)]
pub struct Channel {
    pub channel_id: Id<ChannelMarker>,
    pub guild_id: Id<GuildMarker>,
    pub kind: ChannelType,
    pub name: String,
    pub parent_id: Option<Id<ChannelMarker>>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub position: i32,
}

#[derive(Default)]
pub struct ChannelUpdate {
    pub kind: Option<ChannelType>,
    pub name: Option<String>,
    pub parent_id: Option<Option<Id<ChannelMarker>>>,
    pub permission_overwrites: Option<Option<Vec<PermissionOverwrite>>>,
    pub position: Option<i32>,
}

pub struct CurrentUser {
    pub communication_disabled_until: Option<OffsetDateTime>,
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
    pub role_ids: RwLock<HashSet<Id<RoleMarker>>>,
}

#[derive(Default)]
pub struct CurrentUserUpdate {
    pub communication_disabled_until: Option<Option<OffsetDateTime>>,
    pub role_ids: Option<HashSet<Id<RoleMarker>>>,
}

#[derive(Debug)]
pub struct Guild {
    pub channel_ids: RwLock<HashSet<Id<ChannelMarker>>>,
    pub guild_id: Id<GuildMarker>,
    pub in_check: bool,
    pub invite_check_category_ids: RwLock<HashSet<Id<ChannelMarker>>>,
    pub name: String,
    pub role_ids: RwLock<HashSet<Id<RoleMarker>>>,
}

#[derive(Default)]
pub struct GuildUpdate {
    pub in_check: Option<bool>,
    pub invite_check_category_ids: Option<HashSet<Id<ChannelMarker>>>,
    pub name: Option<String>,
}

pub struct Role {
    pub guild_id: Id<GuildMarker>,
    pub permissions: Permissions,
    pub role_id: Id<RoleMarker>,
}

#[derive(Default)]
pub struct RoleUpdate {
    pub permissions: Option<Permissions>,
}
