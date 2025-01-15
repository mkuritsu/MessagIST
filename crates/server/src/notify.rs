use std::{collections::HashMap, sync::Arc};

use deadqueue::unlimited::Queue;
use protocol::stoc::Message;
use rocket::tokio::sync::RwLock;

pub type MessageQueue = Arc<Queue<Message>>;

pub struct NotifyStore {
    map: RwLock<HashMap<String, MessageQueue>>,
}

impl NotifyStore {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub async fn create_client_queue(&self, client: &str) -> MessageQueue {
        let mut map = self.map.write().await;
        let queue = Arc::new(Queue::new());
        map.insert(client.to_string(), queue.clone());
        queue
    }

    pub async fn remove_client_queue(&self, client: &str) {
        let mut map = self.map.write().await;
        map.remove(client);
    }

    pub async fn get_client_queue(&self, client: &str) -> Option<MessageQueue> {
        let map = self.map.read().await;
        match map.get(client) {
            Some(queue) => Some(queue.clone()),
            None => None,
        }
    }
}

pub struct NotifyService {
    pub store: Arc<NotifyStore>,
}

impl NotifyService {
    pub fn new() -> Self {
        Self {
            store: Arc::new(NotifyStore::new()),
        }
    }
}
