use nitinol::macros::Event;
use serde::{Deserialize, Serialize};
use crate::entities::table::{TableId, TableName};

#[derive(Debug, Clone, Event, Deserialize, Serialize)]
#[persist(enc = "serde_json::to_vec", dec = "serde_json::from_slice")]
pub enum TableEvent {
    Registered {
        id: TableId,
        name: TableName,
    },
    Renamed {
        id: TableId,
        name: TableName,
    },
    Deregistered {
        id: TableId,
    },
}
