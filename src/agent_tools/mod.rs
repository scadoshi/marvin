pub mod math;
pub mod web;
pub use math::math_tools;
use thiserror::Error;
pub use web::WebTools;

use rig::tool::ToolError;

pub(super) trait ToToolError: std::error::Error + Send + Sync + 'static + Sized {
    fn to_tool_err(self) -> ToolError {
        ToolError::ToolCallError(Box::new(self))
    }
}

impl<E> ToToolError for E where E: std::error::Error + Send + Sync + 'static + Sized {}

pub(super) trait ToToolResult<T, E: ToToolError> {
    fn to_tool_result(self) -> Result<T, ToolError>;
}

impl<T, E: ToToolError> ToToolResult<T, E> for Result<T, E> {
    fn to_tool_result(self) -> Result<T, ToolError> {
        self.map_err(|e| e.to_tool_err())
    }
}

#[derive(Debug, Error)]
#[error("{0}")]
pub(super) struct SomeError(String);
