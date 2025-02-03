use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl AsRef<Uuid> for ProductId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<ProductId> for Uuid {
    fn from(id: ProductId) -> Uuid {
        id.0
    }
}

impl Default for ProductId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
