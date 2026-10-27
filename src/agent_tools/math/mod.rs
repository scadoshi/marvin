use rig::tool::ToolDyn;

use crate::agent_tools::math::add::Add;
use crate::agent_tools::math::divide::Divide;
use crate::agent_tools::math::multiply::Multiply;
use crate::agent_tools::math::subtract::Subtract;

pub mod add;
pub mod divide;
pub mod multiply;
pub mod subtract;

pub fn tools() -> Vec<Box<dyn ToolDyn>> {
    vec![
        Box::new(Add),
        Box::new(Subtract),
        Box::new(Multiply),
        Box::new(Divide),
    ]
}
