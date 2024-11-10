#![no_std]
pub mod traits;

use traits::{
    buzzer::Buzzer, display::Display, input_method::InputMethod, light_sensor::LightSensor,
    rtc::RTC,
};

pub struct Watch<D, R, I, B, L>
where
    D: Display,
    R: RTC,
    I: InputMethod,
    B: Buzzer,
    L: LightSensor,
{
    display: D,
    rtc: R,
    input: I,
    buzzer: B,
    light_sensor: L,
}

impl<D, R, I, B, L> Watch<D, R, I, B, L>
where
    D: Display,
    R: RTC,
    I: InputMethod,
    B: Buzzer,
    L: LightSensor,
{
    pub fn new(display: D, rtc: R, input: I, buzzer: B, light_sensor: L) -> Self {
        let mut w = Self {
            display,
            rtc,
            input,
            buzzer,
            light_sensor,
        };
        w.display.print_string(" Hello! ");
        w
    }
}
