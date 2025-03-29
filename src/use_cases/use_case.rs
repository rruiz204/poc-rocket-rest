use rocket::async_trait;

#[async_trait]
pub trait UseCase<Input, Output> {
    async fn execute(&self, input: Input) -> Output;
}