use super::unix::now_unix;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Created,
    Running,
    Stopped,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    pub status: Status,
    pub pid: Option<i32>,
    pub created_at_unix: u64,
    pub started_at_unix: Option<u64>,
}

impl State {
    pub fn new_created(id: String) -> Self {
        Self {
            id,
            status: Status::Created,
            pid: None,
            created_at_unix: now_unix(),
            started_at_unix: None,
        }
    }
}
