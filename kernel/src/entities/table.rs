mod id;
mod name;

pub use self::id::*;
pub use self::name::*;

use std::convert::Infallible;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use error_stack::Report;
use nitinol::{EntityId, ToEntityId};
use nitinol::process::eventstream::WithStreamPublisher;
use nitinol::process::persistence::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::Projection;
use nitinol::projection::resolver::{Mapper, ResolveMapping};

use crate::errors::ValidationError;
use crate::io::command::table::TableCommand;
use crate::io::event::table::TableEvent;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Table {
    id: TableId,
    name: TableName,
}

impl Table {
    pub fn new(id: TableId, name: TableName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &TableId {
        &self.id
    }

    pub fn name(&self) -> &TableName {
        &self.name
    }
}

impl Process for Table {}

impl WithPersistence for Table {
    fn aggregate_id(&self) -> EntityId {
        self.id.to_entity_id()
    }
}

impl WithStreamPublisher for Table {
    fn aggregate_id(&self) -> EntityId {
        self.id.to_entity_id()
    }
}

#[async_trait]
impl Publisher<TableCommand> for Table {
    type Event = TableEvent;
    type Rejection = Report<ValidationError>;

    #[tracing::instrument(skip_all, fields(id = %self.id))]
    async fn publish(&self, command: TableCommand, _: &mut Context) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            TableCommand::Register { name } => {
                TableEvent::Registered { id: self.id, name }
            }
            TableCommand::Rename { name } => {
                TableEvent::Renamed { id: self.id, name }
            }
            TableCommand::Deregister => {
                TableEvent::Deregistered { id: self.id }
            }
        };
        Ok(ev)
    }
}

#[async_trait]
impl Applicator<TableEvent> for Table {
    #[tracing::instrument(skip_all, fields(id = %self.id))]
    async fn apply(&mut self, event: TableEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;
        WithStreamPublisher::publish(self, &event, ctx).await;

        tracing::debug!("Applying event: {:?}", event);
        
        match event {
            TableEvent::Registered { name, .. } => {
                self.name = name;
            }
            TableEvent::Renamed { name, .. } => {
                self.name = name;
            }
            TableEvent::Deregistered { .. } => {
                ctx.poison_pill().await;
            }
        }
        
        tracing::debug!("Event applied, current state: {:?}", self);
    }
}

impl ResolveMapping for Table {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<TableEvent>();
    }
}

#[async_trait]
impl Projection<TableEvent> for Table {
    type Rejection = Infallible;

    async fn first(event: TableEvent) -> Result<Self, Self::Rejection> {
        let TableEvent::Registered { id, name } = event else {
            panic!("Invalid event");
        };
        Ok(Table::new(id, name))
    }

    async fn apply(&mut self, event: TableEvent) -> Result<(), Self::Rejection> {
        match event {
            TableEvent::Registered { name, .. } => {
                self.name = name;
            }
            TableEvent::Renamed { name, .. } => {
                self.name = name;
            }
            TableEvent::Deregistered { .. } => {
                tracing::warn!(id = %self.id, "Table is deregistered, no more changes allowed");
                panic!("Table is deregistered, no more changes allowed");
            }
        }
        Ok(())
    }
}