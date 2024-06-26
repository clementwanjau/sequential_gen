use thiserror::Error;

/// The error type for this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred while locking the mutex.
    #[error("{0}")]
    PoisonError(#[from] std::sync::PoisonError<std::sync::MutexGuard<'static, usize>>),
}
