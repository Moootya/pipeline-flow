use async_trait::async_trait;


pub struct RetryPolicy {
    pub attempts: u32,
    pub sleep_time: f32,
}


pub struct NodeResult<T> {
    pub data: T,
}


#[async_trait]
pub trait Node {
    fn get_retry_policy(&self) -> RetryPolicy {RetryPolicy{attempts: 1, sleep_time: 0.0}}
    fn get_name(&self) -> &'static str;
    fn get_verbose_name(&self) -> &'static str;

    type Input;
    type Output;
    async fn process(&self, input: Self::Input) -> Self::Output;
}


#[macro_export]
macro_rules! as_output {
    ($target_node:ident) => {
        <$target_node as Node>::Output
    };
}
