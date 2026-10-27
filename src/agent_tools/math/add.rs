//! The `add` tool.

use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

use crate::agent_tools::{ToolErrorExt, ToolFailure};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub(crate) struct AddArgs {
    #[schemars(description = "left hand side \\ the first number")]
    pub lhs: i64,
    #[schemars(description = "right hand side \\ the second number")]
    pub rhs: i64,
}

/// Adds two `i64`s.
#[derive(Debug)]
pub(crate) struct Add;

#[expect(
    clippy::unused_async_trait_impl,
    reason = "rig's Tool trait declares definition and call async"
)]
impl Tool for Add {
    const NAME: &'static str = "add";
    type Args = AddArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "returns sum of two given numbers, lhs and rhs".to_string(),
            parameters: schema_for!(AddArgs).into(),
        }
    }

    /// Returns `lhs + rhs`.
    ///
    /// # Errors
    ///
    /// Returns an error if the sum overflows `i64`.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        args.lhs
            .checked_add(args.rhs)
            .ok_or_else(|| ToolFailure(String::from("overflow")).into_tool_error())
    }
}
