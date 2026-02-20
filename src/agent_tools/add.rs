use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolError},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddArgs {
    pub lhs: i64,
    pub rhs: i64,
}

pub struct Adder;

impl Tool for Adder {
    const NAME: &'static str = "add_numbers";
    type Args = AddArgs;
    type Output = i64;
    type Error = ToolError;

    async fn definition(&self, _promp: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "returns sum of two given numbers, lhs and rhs".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "lhs": {"type": "number", "description": "left hand side: the first number"},
                    "rhs": {"type": "number", "description": "right hand side: the second number"}
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args.lhs + args.rhs)
    }
}
