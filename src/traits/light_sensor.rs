#[allow(async_fn_in_trait)]
pub trait LightSensor {
    type ReturnType;

    async fn get_light_intensity(&mut self) -> Self::ReturnType;
}
