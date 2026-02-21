use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AddArgs {
    #[schemars(description = "left hand side \\ the first number")]
    pub lhs: i64,
    #[schemars(description = "right hand side \\ the second number")]
    pub rhs: i64,
}

pub struct Add;

impl Tool for Add {
    const NAME: &'static str = "add";
    type Args = AddArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "returns sum of two given numbers, lhs and rhs".to_string(),
            parameters: serde_json::to_value(schema_for!(AddArgs)).unwrap(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.lhs + args.rhs)
    }
}
