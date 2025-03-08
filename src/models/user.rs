use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub permissions: HashSet<String>,
}

impl Default for User {
    fn default() -> Self {
        let permissions = HashSet::new();

        Self {
            id: -1,
            username: "Guest".into(),
            permissions,
        }
    }
}
