use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

use crate::agent_tools::{ToolErrorExt, ToolFailure};

#[derive(Serialize, Deserialize, JsonSchema)]
pub(crate) struct AddArgs {
    #[schemars(description = "left hand side \\ the first number")]
    pub lhs: i64,
    #[schemars(description = "right hand side \\ the second number")]
    pub rhs: i64,
}

pub(crate) struct Add;

// rig's Tool trait declares these methods async.
#[allow(clippy::unused_async_trait_impl)]
impl Tool for Add {
    const NAME: &'static str = "add";
    type Args = AddArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "returns sum of two given numbers, lhs and rhs".to_string(),
            parameters: serde_json::to_value(schema_for!(AddArgs)).expect("schema is serializable"),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        args.lhs
            .checked_add(args.rhs)
            .ok_or_else(|| ToolFailure(String::from("overflow")).into_tool_error())
    }
}
