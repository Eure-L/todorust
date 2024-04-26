use std::fmt::{Display, Formatter};
use serde::Deserialize;
use tokio::net::unix::uid_t;


#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum TaskStatus {
    Pending,
    Started,
    Completed,
    Abandoned
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => {
                write!(f, "Pending")
            }
            TaskStatus::Started => {
                write!(f, "Started")
            }
            TaskStatus::Completed => {
                write!(f, "Completed")
            }
            TaskStatus::Abandoned => {
                write!(f, "Abandoned")
            }
        }

    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub id: uid_t,
    pub description: String,
    pub status: TaskStatus
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id: {})", self.name, self.id)
    }
}

