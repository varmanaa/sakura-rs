mod channel;
mod current_user;
mod guild;
mod role;
mod unavailable_guild;

use std::collections::{HashMap, HashSet};

use parking_lot::RwLock;

use crate::types::cache::Cache;

impl Cache {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            current_users: RwLock::new(HashMap::new()),
            guilds: RwLock::new(HashMap::new()),
            roles: RwLock::new(HashMap::new()),
            unavailable_guilds: RwLock::new(HashSet::new()),
        }
    }
}
