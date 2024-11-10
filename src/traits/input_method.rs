use embassy_time::Duration;

pub enum Input {
    Up(Duration),
    Down(Duration),
    Left(Duration),
    Right(Duration),
    Confirm(Duration),
    Cancel(Duration),
}

#[allow(async_fn_in_trait)]
pub trait InputMethod {
    async fn wait_for_input(
        &mut self,
        min_time: Option<Duration>,
        timeout: Option<Duration>,
    ) -> Option<Input>;
    async fn wait_for_no_input(&mut self, timeout: Option<Duration>) -> Result<(), ()>;
}
