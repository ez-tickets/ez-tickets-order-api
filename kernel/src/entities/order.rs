mod id;
mod product_id;
mod quantity;

pub use self::id::*;
pub use self::product_id::*;
pub use self::quantity::*;

use async_trait::async_trait;
use error_stack::Report;
use nitinol::process::eventstream::WithStreamPublisher;
use nitinol::process::persistence::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::resolver::{Mapper, ResolveMapping};
use nitinol::projection::Projection;
use nitinol::{EntityId, ToEntityId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;

use crate::entities::table::TableId;
use crate::errors::ValidationError;
use crate::io::command::order::OrderCommand;
use crate::io::event::order::OrderEvent;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Order {
    id: OrderId,
    table: TableId,
    products: Vec<HashMap<ProductId, Quantity>>,
}

impl Order {
    pub fn new(id: OrderId, table: TableId) -> Order {
        Order {
            id,
            table,
            products: Default::default(),
        }
    }

    pub fn id(&self) -> &OrderId {
        &self.id
    }

    pub fn table(&self) -> &TableId {
        &self.table
    }

    pub fn products(&self) -> &[HashMap<ProductId, Quantity>] {
        &self.products
    }
}

impl Process for Order {}

impl WithPersistence for Order {
    fn aggregate_id(&self) -> EntityId {
        self.id.to_entity_id()
    }
}

impl WithStreamPublisher for Order {
    fn aggregate_id(&self) -> EntityId {
        self.id.to_entity_id()
    }
}

#[async_trait]
impl Publisher<OrderCommand> for Order {
    type Event = OrderEvent;
    type Rejection = Report<ValidationError>;

    #[tracing::instrument(skip_all, fields(id = %self.id, table = %self.table))]
    async fn publish(
        &self,
        command: OrderCommand,
        _: &mut Context,
    ) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            OrderCommand::Create { table } => OrderEvent::Created { id: self.id, table },
            OrderCommand::AddProducts { products } => OrderEvent::AddedProducts {
                id: self.id,
                products,
            },
            OrderCommand::Settle => OrderEvent::Settled {
                id: self.id,
                table_id: self.table,
            },
        };
        tracing::debug!("Accepted command");
        Ok(ev)
    }
}

#[async_trait]
impl Applicator<OrderEvent> for Order {
    #[tracing::instrument(skip_all, fields(id = %self.id, table = %self.table))]
    async fn apply(&mut self, event: OrderEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;
        WithStreamPublisher::publish(self, &event, ctx).await;

        tracing::debug!("Applying event: {:?}", event);

        match event {
            OrderEvent::AddedProducts { products, .. } => {
                self.products.push(products);
            }
            OrderEvent::Settled { .. } => {
                ctx.poison_pill().await;
            }
            _ => {
                tracing::debug!("ignore event. {:?}", event);
            }
        }

        tracing::debug!("Event applied, current state: {:?}", self);
    }
}

impl ResolveMapping for Order {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<OrderEvent>();
    }
}

#[async_trait]
impl Projection<OrderEvent> for Order {
    type Rejection = Infallible;
    async fn first(event: OrderEvent) -> Result<Self, Self::Rejection> {
        let OrderEvent::Created { id, table } = event else {
            panic!("Invalid event");
        };
        Ok(Order::new(id, table))
    }

    async fn apply(&mut self, event: OrderEvent) -> Result<(), Self::Rejection> {
        match event {
            OrderEvent::AddedProducts { products, .. } => {
                self.products.push(products);
            }
            OrderEvent::Settled { id, table_id } => {
                tracing::warn!(id = %id, table = %table_id, "Order is settled, no more changes allowed");
                panic!("Order is settled, no more changes allowed");
            }
            _ => {
                tracing::debug!("ignore projection event. {:?}", event);
            }
        }
        Ok(())
    }
}
