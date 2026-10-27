use rig::tool::ToolDyn;

use crate::agent_tools::math::add::Add;
use crate::agent_tools::math::divide::Divide;
use crate::agent_tools::math::multiply::Multiply;
use crate::agent_tools::math::subtract::Subtract;

pub(crate) mod add;
pub(crate) mod divide;
pub(crate) mod multiply;
pub(crate) mod subtract;

pub(crate) fn tools() -> Vec<Box<dyn ToolDyn>> {
    vec![
        Box::new(Add),
        Box::new(Subtract),
        Box::new(Multiply),
        Box::new(Divide),
    ]
}

#[cfg(test)]
mod tests {
    use rig::tool::Tool;

    use super::add::{Add, AddArgs};
    use super::divide::{Divide, DivideArgs};
    use super::multiply::{Multiply, MultiplyArgs};
    use super::subtract::{Subtract, SubtractArgs};

    #[tokio::test]
    async fn each_tool_does_its_arithmetic() {
        assert_eq!(Add.call(AddArgs { lhs: 7, rhs: 3 }).await.unwrap(), 10);
        assert_eq!(
            Subtract
                .call(SubtractArgs { lhs: 7, rhs: 3 })
                .await
                .unwrap(),
            4
        );
        assert_eq!(
            Multiply
                .call(MultiplyArgs { lhs: 7, rhs: 3 })
                .await
                .unwrap(),
            21
        );
        assert_eq!(Divide.call(DivideArgs { lhs: 7, rhs: 3 }).await.unwrap(), 2);
    }

    #[tokio::test]
    async fn dividing_by_zero_is_an_error() {
        let err = Divide
            .call(DivideArgs { lhs: 1, rhs: 0 })
            .await
            .unwrap_err();
        assert!(err.to_string().contains("division by zero"), "{err}");
    }

    #[tokio::test]
    async fn overflow_is_an_error() {
        let errors = [
            Add.call(AddArgs {
                lhs: i64::MAX,
                rhs: 1,
            })
            .await,
            Subtract
                .call(SubtractArgs {
                    lhs: i64::MIN,
                    rhs: 1,
                })
                .await,
            Multiply
                .call(MultiplyArgs {
                    lhs: i64::MAX,
                    rhs: 2,
                })
                .await,
            Divide
                .call(DivideArgs {
                    lhs: i64::MIN,
                    rhs: -1,
                })
                .await,
        ];
        for result in errors {
            let err = result.unwrap_err();
            assert!(err.to_string().contains("overflow"), "{err}");
        }
    }
}
