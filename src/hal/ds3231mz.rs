use embassy_stm32::{
    gpio::{AnyPin, Level, Output, Speed},
    i2c::I2c,
    peripherals::{DMA1_CH2, DMA1_CH3, I2C1},
};

use stm32watch_rs::traits::rtc::{Date, Time, RTC};

const I2C_ADDRESS: u8 = 0x68;

#[repr(u8)]
#[allow(dead_code)]
enum Registers {
    Second = 0x0,
    Minute,
    Hour,
    Date,
    Month,
    Year,
}

pub struct DS3231mz {
    en_pin: Output<'static, AnyPin>,
    i2c_bus: I2c<'static, I2C1, DMA1_CH2, DMA1_CH3>,
}

impl DS3231mz {
    pub fn new(en_pin: AnyPin, i2c_bus: I2c<'static, I2C1, DMA1_CH2, DMA1_CH3>) -> Self {
        Self {
            en_pin: Output::new(en_pin, Level::High, Speed::Medium),
            i2c_bus,
        }
    }
}

impl RTC for DS3231mz {
    type Error = embassy_stm32::i2c::Error;

    async fn get_time(&mut self) -> Result<Time, Self::Error> {
        let mut buffer = [0u8; 3];
        self.i2c_bus
            .write_read(I2C_ADDRESS, &[Registers::Second as u8], &mut buffer)
            .await?;
        Ok(Time {
            second: bcd_to_byte(buffer[0]),
            minute: bcd_to_byte(buffer[1]),
            hour: bcd_to_byte(buffer[2]),
        })
    }

    async fn get_date(&mut self) -> Result<Date, Self::Error> {
        let mut buffer = [0u8; 3];
        self.i2c_bus
            .write_read(I2C_ADDRESS, &[Registers::Date as u8], &mut buffer)
            .await?;
        Ok(Date {
            date: bcd_to_byte(buffer[0]),
            month: bcd_to_byte(buffer[1]),
            year: bcd_to_byte(buffer[2]),
        })
    }

    async fn set_time(&mut self, new_time: Time) -> Result<(), Self::Error> {
        self.i2c_bus
            .write(
                I2C_ADDRESS,
                &[
                    Registers::Second as u8,
                    byte_to_bcd(new_time.second),
                    byte_to_bcd(new_time.minute),
                    byte_to_bcd(new_time.hour),
                ],
            )
            .await
    }

    async fn set_date(&mut self, new_date: Date) -> Result<(), Self::Error> {
        self.i2c_bus
            .write(
                I2C_ADDRESS,
                &[
                    Registers::Date as u8,
                    byte_to_bcd(new_date.date),
                    byte_to_bcd(new_date.month),
                    byte_to_bcd(new_date.year),
                ],
            )
            .await
    }
}

fn bcd_to_byte(bcd_data: u8) -> u8 {
    ((bcd_data >> 4) * 10) + (bcd_data % 16)
}

fn byte_to_bcd(byte: u8) -> u8 {
    ((byte / 10) << 4) | (byte % 10)
}
