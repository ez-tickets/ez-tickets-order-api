use crate::entities::order::{Order, OrderId, ProductId, Quantity};
use crate::entities::table::TableId;
use nitinol::macros::Command;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use error_stack::Report;
use crate::errors::FormationError;

#[derive(Debug, Clone, Command, Deserialize, Serialize)]
pub enum OrderCommand {
    Create {
        table: TableId,
    },
    AddProducts {
        products: HashMap<ProductId, Quantity>,
    },
    Settle,
}

impl TryFrom<(OrderId, OrderCommand)> for Order {
    type Error = Report<FormationError>;

    fn try_from((id, cmd): (OrderId, OrderCommand)) -> Result<Self, Self::Error> {
        match cmd {
            OrderCommand::Create { table } => Ok(Order::new(id, table)),
            _ => Err(Report::new(FormationError).attach_printable("Invalid command")),
        }
    }
}