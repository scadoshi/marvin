//! The `subtract` tool.

use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

use crate::agent_tools::{ToolErrorExt, ToolFailure};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub(crate) struct SubtractArgs {
    #[schemars(description = "left hand side \\ the first number \\ the minuend")]
    pub lhs: i64,
    #[schemars(description = "right hand side \\ the second number \\ the subtrahend")]
    pub rhs: i64,
}

/// Subtracts two `i64`s.
#[derive(Debug)]
pub(crate) struct Subtract;

#[expect(
    clippy::unused_async_trait_impl,
    reason = "rig's Tool trait declares definition and call async"
)]
impl Tool for Subtract {
    const NAME: &'static str = "subtract";
    type Args = SubtractArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "returns difference of two given numbers, lhs and rhs (rhs is subtracted from lhs)"
                    .to_string(),
            parameters: schema_for!(SubtractArgs).into(),
        }
    }

    /// Returns `lhs - rhs`.
    ///
    /// # Errors
    ///
    /// Returns an error if the difference overflows `i64`.
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        args.lhs
            .checked_sub(args.rhs)
            .ok_or_else(|| ToolFailure(String::from("overflow")).into_tool_error())
    }
}
