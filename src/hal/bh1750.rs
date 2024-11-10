use stm32watch_rs::traits::light_sensor::LightSensor;

pub struct BH1750 {}

impl LightSensor for BH1750 {
    type ReturnType = u16;

    async fn get_light_intensity(&mut self) -> Self::ReturnType {
        todo!()
    }
}
