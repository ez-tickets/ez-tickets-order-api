#[derive(Debug, thiserror::Error)]
#[error("Something went wrong...")]
pub struct UnrecoverableError;