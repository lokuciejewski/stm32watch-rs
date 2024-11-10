use stm32watch_rs::traits::buzzer::Buzzer;


pub struct VibrationBuzzer {

}

impl Buzzer for VibrationBuzzer {
    async fn buzz(time: embassy_time::Duration) {
        todo!()
    }
}