use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Quantity(i32);

impl Quantity {
    pub fn new(qty: impl Into<i32>) -> Self {
        Self(qty.into())
    }
}

impl AsRef<i32> for Quantity {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<Quantity> for i32 {
    fn from(qty: Quantity) -> i32 {
        qty.0
    }
}
