use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::models::Task;

#[derive(Debug, Default)]
pub struct TaskStore {
    tasks: HashMap<Uuid, Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed() -> Self {
        let mut store = Self::default();
        let now = Utc::now();
        store.insert(Task {
            id: Uuid::new_v4(),
            title: "Write the README".to_string(),
            status: "todo".to_string(),
            created_at: now,
        });
        store.insert(Task {
            id: Uuid::new_v4(),
            title: "Add tests for the status filter".to_string(),
            status: "in-progress".to_string(),
            created_at: now,
        });
        store.insert(Task {
            id: Uuid::new_v4(),
            title: "Ship v0.1".to_string(),
            status: "done".to_string(),
            created_at: now,
        });
        store
    }

    pub fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    pub fn get(&self, id: &Uuid) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
}
