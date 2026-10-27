use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

use crate::agent_tools::{ToolErrorExt, ToolFailure};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub(crate) struct MultiplyArgs {
    #[schemars(description = "left hand side \\ the first number")]
    pub lhs: i64,
    #[schemars(description = "right hand side \\ the second number")]
    pub rhs: i64,
}

#[derive(Debug)]
pub(crate) struct Multiply;

#[expect(
    clippy::unused_async_trait_impl,
    reason = "rig's Tool trait declares definition and call async"
)]
impl Tool for Multiply {
    const NAME: &'static str = "multiply";
    type Args = MultiplyArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "returns product of two given numbers, lhs and rhs".to_string(),
            parameters: schema_for!(MultiplyArgs).into(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        args.lhs
            .checked_mul(args.rhs)
            .ok_or_else(|| ToolFailure(String::from("overflow")).into_tool_error())
    }
}
