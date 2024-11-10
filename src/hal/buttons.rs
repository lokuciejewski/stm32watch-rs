use embassy_stm32::gpio::AnyPin;

use embassy_time::Timer;
use stm32watch_rs::traits::input_method::{Input, InputMethod};

pub struct Buttons {
    left: embassy_stm32::gpio::Input<'static, AnyPin>,
    center: embassy_stm32::gpio::Input<'static, AnyPin>,
    right: embassy_stm32::gpio::Input<'static, AnyPin>,
}

impl Buttons {
    pub fn new(left: AnyPin, center: AnyPin, right: AnyPin) -> Self {
        Self {
            left: embassy_stm32::gpio::Input::new(left, embassy_stm32::gpio::Pull::Up),
            center: embassy_stm32::gpio::Input::new(center, embassy_stm32::gpio::Pull::Up),
            right: embassy_stm32::gpio::Input::new(right, embassy_stm32::gpio::Pull::Up),
        }
    }
}

impl InputMethod for Buttons {
    async fn wait_for_input(
        &mut self,
        min_time: Option<embassy_time::Duration>,
        timeout: Option<embassy_time::Duration>,
    ) -> Option<Input> {
        todo!()
    }

    async fn wait_for_no_input(
        &mut self,
        timeout: Option<embassy_time::Duration>,
    ) -> Result<(), ()> {
        todo!()
    }
}
