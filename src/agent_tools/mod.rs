pub mod math;
pub mod web;
use rig::tool::ToolError;
use thiserror::Error;

pub(super) trait ToolErrorExt: std::error::Error + Send + Sync + 'static + Sized {
    fn into_tool_error(self) -> ToolError {
        ToolError::ToolCallError(Box::new(self))
    }
}

impl<E> ToolErrorExt for E where E: std::error::Error + Send + Sync + 'static + Sized {}

pub(super) trait ToolResultExt<T, E: ToolErrorExt> {
    fn into_tool_result(self) -> Result<T, ToolError>;
}

impl<T, E: ToolErrorExt> ToolResultExt<T, E> for Result<T, E> {
    fn into_tool_result(self) -> Result<T, ToolError> {
        self.map_err(ToolErrorExt::into_tool_error)
    }
}

#[derive(Debug, Error)]
#[error("{0}")]
pub(super) struct ToolFailure(String);
