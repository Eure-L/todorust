use std::fmt::{Display, Formatter};
use tokio::net::unix::uid_t;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub id: uid_t,
    pub description: String,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id: {})", self.name, self.id)
    }
}
