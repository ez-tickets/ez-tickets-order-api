use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableName(String);

impl TableName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl AsRef<str> for TableName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<TableName> for String {
    fn from(name: TableName) -> Self {
        name.0
    }
}
