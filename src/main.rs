#![no_std]
#![no_main]
pub mod hal;
pub mod traits;

extern crate defmt;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    gpio::Pin,
    i2c::{self, I2c},
    peripherals::I2C1,
    time::Hertz,
};
use hal::{
    bh1750::BH1750, buttons::Buttons, ds3231mz::DS3231mz, hcms_2904::Hcms2904,
    vibration_buzzer::VibrationBuzzer,
};
use stm32watch_rs::{traits::display::Display, Watch};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1 => i2c::EventInterruptHandler<I2C1>, i2c::ErrorInterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut display = Hcms2904::new(
        p.PA1.degrade(),
        p.PA3.degrade(),
        p.PA5.degrade(),
        p.PA4.degrade(),
        p.PA6.degrade(),
    );

    display.init_display().await;
    display.clear();

    let i2c = I2c::new(
        p.I2C1,
        p.PA9,
        p.PA10,
        Irqs,
        p.DMA1_CH2,
        p.DMA1_CH3,
        Hertz(100_000),
        Default::default(),
    );

    let rtc = DS3231mz::new(p.PA8.degrade(), i2c);

    let buttons = Buttons::new(p.PB3.degrade(), p.PB5.degrade(), p.PB4.degrade());

    let buzzer = VibrationBuzzer {};
    let light_sensor = BH1750 {};

    let mut watch = Watch::new(display, rtc, buttons, buzzer, light_sensor);
    // rtc.set_time(Time {hour: 22, minute: 17, second: 20}).await.unwrap();

    // let mut ticker = Ticker::every(Duration::from_hz(1));

    loop {
        // let mut time_str: heapless::String<8> = heapless::String::new();
        // match rtc.get_time().await {
        //     Ok(time) => {
        //         info!("Got time: {}", time);
        //         write!(time_str, "{}", time).unwrap();
        //         // d.print_string(time_str.as_str());
        //     }
        //     Err(err) => error!("I2C error: {}", err),
        // }
        // ticker.next().await;
    }
}
