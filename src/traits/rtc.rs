use core::fmt::Display;

use defmt::Format;

#[derive(Debug)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Display for Time {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{:02}:{:02}:{:02}",
            self.hour, self.minute, self.second
        ))
    }
}

impl Format for Time {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "{:02}:{:02}:{:02}",
            self.hour,
            self.minute,
            self.second
        );
    }
}

#[derive(Debug)]
pub struct Date {
    pub date: u8,
    pub month: u8,
    pub year: u8,
}

impl Format for Date {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{:02}/{:02}/{:02}", self.date, self.month, self.year)
    }
}

#[allow(async_fn_in_trait)]
pub trait RTC {
    type Error;

    async fn get_time(&mut self) -> Result<Time, Self::Error>;
    async fn get_date(&mut self) -> Result<Date, Self::Error>;

    async fn set_time(&mut self, new_time: Time) -> Result<(), Self::Error>;
    async fn set_date(&mut self, new_date: Date) -> Result<(), Self::Error>;
}
