#[macro_use]
extern crate pipeline_flow;

use async_trait::async_trait;
use tokio;
use pipeline_flow::protocol::{Node};


struct FirstNode {}

#[async_trait]
impl Node for FirstNode {
    fn get_name(&self) -> &'static str { "example-id" }
    fn get_verbose_name(&self) -> &'static str { "example-name" }

    type Input = u32;
    type Output = u32;

    async fn process(&self, input: Self::Input) -> Self::Output {
        return input
    }
}

struct SecondNode {}

#[async_trait]
impl Node for SecondNode {
    fn get_name(&self) -> &'static str { "second-example-id" }
    fn get_verbose_name(&self) -> &'static str { "second-example-name" }

    type Input = as_output!(FirstNode);
    type Output = u32;

    async fn process(&self, input: Self::Input) -> Self::Output {
        return input + 1
    }
}


struct ThirdNode {}

#[async_trait]
impl Node for ThirdNode {
    fn get_name(&self) -> &'static str { "second-example-id" }
    fn get_verbose_name(&self) -> &'static str { "second-example-name" }

    type Input = (as_output!(FirstNode), as_output!(SecondNode));
    type Output = u32;

    async fn process(&self, input: Self::Input) -> Self::Output {
        return input.0 + input.1 + 1
    }
}


#[tokio::test]
async fn test_node_impl() {

    let node = FirstNode{};
    let node_result = node.process(123).await;

    assert_eq!(node_result, 123);

    assert_eq!(node.get_name(), "example-id");
    assert_eq!(node.get_verbose_name(), "example-name");
    assert_eq!(node.get_retry_policy().sleep_time, 0.0);
    assert_eq!(node.get_retry_policy().attempts, 1);
}


#[tokio::test]
async fn test_node_connection() {

    let first_node = FirstNode{};
    let first_node_result = first_node.process(123).await;

    let second_node = SecondNode{};
    let second_node_result = second_node.process(first_node_result).await;

    let third_node = ThirdNode{};
    let third_node_result = third_node.process((first_node_result, second_node_result)).await;

    assert_eq!(third_node_result, 248);
}
