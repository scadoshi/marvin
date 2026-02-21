use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DivideArgs {
    #[schemars(description = "left hand side \\ the first number \\ the dividend")]
    pub lhs: i64,
    #[schemars(description = "right hand side \\ the second number \\ the divisor")]
    pub rhs: i64,
}

pub struct Divide;

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
            parameters: serde_json::to_value(schema_for!(DivideArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.lhs / args.rhs)
    }
}
