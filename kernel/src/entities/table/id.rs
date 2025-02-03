use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct TableId(Uuid);

impl TableId {
    pub fn new(id: impl Into<Uuid>) -> TableId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for TableId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<TableId> for Uuid {
    fn from(id: TableId) -> Uuid {
        id.0
    }
}

impl Display for TableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for TableId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
