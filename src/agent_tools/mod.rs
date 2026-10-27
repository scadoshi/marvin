//! Tools the model can call, and helpers that turn their errors into rig's [`ToolError`].

pub(crate) mod math;
pub(crate) mod web;
use rig::tool::ToolError;
use thiserror::Error;

/// Converts any thread-safe error into a [`ToolError`].
pub(super) trait ToolErrorExt: std::error::Error + Send + Sync + 'static + Sized {
    /// Boxes `self` as a [`ToolError::ToolCallError`].
    fn into_tool_error(self) -> ToolError {
        ToolError::ToolCallError(Box::new(self))
    }
}

impl<E> ToolErrorExt for E where E: std::error::Error + Send + Sync + 'static + Sized {}

/// Converts the error side of a `Result` into a [`ToolError`].
pub(super) trait ToolResultExt<T, E: ToolErrorExt> {
    /// Wraps the error, if any, as a [`ToolError::ToolCallError`].
    ///
    /// # Errors
    ///
    /// Returns an error if `self` is an error.
    fn into_tool_result(self) -> Result<T, ToolError>;
}

impl<T, E: ToolErrorExt> ToolResultExt<T, E> for Result<T, E> {
    fn into_tool_result(self) -> Result<T, ToolError> {
        self.map_err(ToolErrorExt::into_tool_error)
    }
}

/// A tool failure that is only a message, for cases with no error type of their own.
#[derive(Debug, Error)]
#[error("{0}")]
pub(super) struct ToolFailure(String);
