#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("The resource could not be found or the `Projection` may have failed.")]
    Formation,

    #[error("Failed to spawn process")]
    Process,

    #[error("Request requires an identifier")]
    RequiredId,
    
    #[error("Failed to query external api.")]
    Io,
    
    #[error("Resource not found.")]
    NotFound,

    #[error("An error occurred due to kernel module")]
    Kernel
}