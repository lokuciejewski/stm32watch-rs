use embassy_time::Duration;

#[allow(async_fn_in_trait)]
pub trait Buzzer {
    async fn buzz(time: Duration);
}
