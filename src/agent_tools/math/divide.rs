use rig::completion::ToolDefinition;
use rig::tool::{Tool, ToolError};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

use crate::agent_tools::{ToolErrorExt, ToolFailure};

#[derive(Serialize, Deserialize, JsonSchema)]
pub(crate) struct DivideArgs {
    #[schemars(description = "left hand side \\ the first number \\ the dividend")]
    pub lhs: i64,
    #[schemars(description = "right hand side \\ the second number \\ the divisor")]
    pub rhs: i64,
}

pub(crate) struct Divide;

// rig's Tool trait declares these methods async.
#[allow(clippy::unused_async_trait_impl)]
impl Tool for Divide {
    const NAME: &'static str = "divide";
    type Args = DivideArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "returns quotient of two given numbers, lhs and rhs (lhs is divided by rhs)"
                    .to_string(),
            parameters: schema_for!(DivideArgs).into(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if args.rhs == 0 {
            return Err(ToolFailure(String::from("division by zero")).into_tool_error());
        }
        args.lhs
            .checked_div(args.rhs)
            .ok_or_else(|| ToolFailure(String::from("overflow")).into_tool_error())
    }
}
