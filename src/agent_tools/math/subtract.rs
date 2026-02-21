use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SubtractArgs {
    #[schemars(description = "left hand side \\ the first number \\ the minuend")]
    pub lhs: i64,
    #[schemars(description = "right hand side \\ the second number \\ the subtrahend")]
    pub rhs: i64,
}

pub struct Subtract;

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
            parameters: serde_json::to_value(schema_for!(SubtractArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.lhs - args.rhs)
    }
}
