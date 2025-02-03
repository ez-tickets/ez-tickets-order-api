use nitinol::macros::Command;
use serde::{Deserialize, Serialize};
use crate::entities::table::TableName;

#[derive(Debug, Clone, Command, Deserialize, Serialize)]
pub enum TableCommand {
    Register {
        name: TableName,
    },
    Rename {
        name: TableName,
    },
    Deregister,
}
