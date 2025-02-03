use crate::entities::order::{OrderId, ProductId, Quantity};
use crate::entities::table::TableId;
use nitinol::macros::Event;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Event, Deserialize, Serialize)]
#[persist(enc = "serde_json::to_vec", dec = "serde_json::from_slice")]
pub enum OrderEvent {
    Created {
        id: OrderId,
        table: TableId,
    },
    AddedProducts {
        id: OrderId,
        products: HashMap<ProductId, Quantity>,
    },
    Settled {
        id: OrderId,
        table_id: TableId,
    },
}
