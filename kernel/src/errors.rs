#[derive(Debug, thiserror::Error)]
#[error("Failed to validate entity.")]
pub struct ValidationError;

#[derive(Debug, thiserror::Error)]
#[error("Failed to inquiry for service.")]
pub struct InquiryError;

#[derive(Debug, thiserror::Error)]
#[error("input cannot be converted.")]
pub struct FormationError;