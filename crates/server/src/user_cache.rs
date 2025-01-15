use std::collections::HashMap;

use rocket::tokio::sync::Mutex;

use crate::db::structs::User;

pub struct UserCache {
    map: HashMap<String, User>,
}

impl UserCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn store(&mut self, user: User) {
        self.map.insert(user.id.clone(), user);
    }

    pub fn get(&self, user_id: &str) -> Option<&User> {
        self.map.get(user_id)
    }
}

pub struct UserCacheService {
    pub cache: Mutex<UserCache>,
}

impl UserCacheService {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(UserCache::new()),
        }
    }
}
